// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// infer_tu_main infers nullability within a single translation unit.
//
// By default (-diagnostics=1) it shows findings as diagnostics.
// It can optionally (-protos=1) print the Inference proto.
//
// This is not the intended way to fully analyze a real codebase.
// e.g. it can't jointly inspect all callsites of a function (in different TUs).

#include <memory>
#include <string>
#include <utility>

#include "absl/log/check.h"
#include "nullability/inference/infer_tu.h"
#include "nullability/inference/inference.proto.h"
#include "clang/AST/ASTConsumer.h"
#include "clang/AST/Decl.h"
#include "clang/AST/RecursiveASTVisitor.h"
#include "clang/Index/USRGeneration.h"
#include "clang/Tooling/ArgumentsAdjusters.h"
#include "clang/Tooling/Execution.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/ArrayRef.h"
#include "llvm/ADT/DenseMap.h"
#include "llvm/ADT/STLExtras.h"
#include "llvm/ADT/SmallString.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/Twine.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/Error.h"
#include "llvm/Support/raw_ostream.h"

llvm::cl::OptionCategory Opts("infer_tu_main options");
llvm::cl::opt<bool> PrintProtos{
    "protos",
    llvm::cl::desc("Print the Inference protos"),
    llvm::cl::init(false),
};
llvm::cl::opt<bool> Diagnostics{
    "diagnostics",
    llvm::cl::desc("Print inference results as diagnostics"),
    llvm::cl::init(true),
};
llvm::cl::opt<bool> PrintEvidence{
    "evidence",
    llvm::cl::desc("Print sample evidence as notes (requires -diagnostics)"),
    llvm::cl::init(true),
};
llvm::cl::opt<bool> IncludeTrivial{
    "trivial",
    llvm::cl::desc("Include trivial inferences (annotated, no conflicts)"),
    llvm::cl::init(false),
};

namespace clang::tidy::nullability {
namespace {

// Walks the AST looking for declarations of symbols we inferred.
// When it finds them, prints the inference as diagnostics.
class DiagnosticPrinter : public RecursiveASTVisitor<DiagnosticPrinter> {
  llvm::DenseMap<llvm::StringRef, const Inference *> InferenceByUSR;
  DiagnosticsEngine &Diags;
  unsigned DiagInferHere;
  unsigned DiagSample;

  void render(const Inference &I, SourceLocation Loc) {
    for (const auto &Slot : I.slot_inference()) {
      Diags.Report(Loc, DiagInferHere)
          << slotName(Slot.slot())
          << Inference::Nullability_Name(Slot.nullability());
      if (PrintEvidence) {
        for (const auto &Sample : Slot.sample_evidence()) {
          if (SourceLocation Loc = parseLoc(Sample.location()); Loc.isValid())
            Diags.Report(Loc, DiagSample) << Evidence::Kind_Name(Sample.kind());
        }
      }
    }
  }

  std::string slotName(unsigned S) {
    if (S == SLOT_RETURN_TYPE) return "return type";
    return ("parameter " + llvm::Twine(S - SLOT_PARAM)).str();
  }

  // Terrible hack: parse "foo.cc:4:2" back into a SourceLocation.
  SourceLocation parseLoc(llvm::StringRef LocStr) {
    auto &SM = Diags.getSourceManager();
    auto &FM = SM.getFileManager();
    auto [Rest, ColStr] = llvm::StringRef(LocStr).rsplit(':');
    auto [Name, LineStr] = Rest.rsplit(':');
    auto File = FM.getOptionalFileRef(Name);
    unsigned Line, Col;
    if (!File || LineStr.getAsInteger(10, Line) || ColStr.getAsInteger(10, Col))
      return SourceLocation();
    return SM.translateFileLineCol(&File->getFileEntry(), Line, Col);
  }

 public:
  DiagnosticPrinter(llvm::ArrayRef<Inference> All, DiagnosticsEngine &Diags)
      : Diags(Diags) {
    for (const auto &I : All) InferenceByUSR.try_emplace(I.symbol().usr(), &I);
    DiagInferHere = Diags.getCustomDiagID(DiagnosticsEngine::Remark,
                                          "would mark %0 as %1 here");
    DiagSample = Diags.getCustomDiagID(DiagnosticsEngine::Note, "%0 here");
  }

  bool VisitDecl(const Decl *FD) {
    llvm::SmallString<128> USR;
    if (!index::generateUSRForDecl(FD, USR))
      if (auto *I = InferenceByUSR.lookup(USR)) render(*I, FD->getLocation());
    return true;
  }
};

bool isTrivial(const Inference::SlotInference &I) {
  if (I.conflict()) return false;
  for (const auto &E : I.sample_evidence())
    if (E.kind() == Evidence::ANNOTATED_NONNULL ||
        E.kind() == Evidence::ANNOTATED_NULLABLE)
      return true;
  return false;
}

void run(ASTContext &Ctx) override {
  llvm::errs() << "Running inference...";
  auto Results = inferTU(Ctx);
  if (!IncludeTrivial)
    llvm::erase_if(Results, [](Inference &I) {
      llvm::erase_if(*I.mutable_slot_inference(), isTrivial);
      return I.slot_inference_size() == 0;
    });
  if (PrintProtos)
    for (const auto &I : Results) llvm::outs() << I.DebugString() << "\n";
  if (Diagnostics)
    DiagnosticPrinter(Results, Ctx.getDiagnostics()).TraverseAST(Ctx);
}

}  // namespace
}  // namespace clang::tidy::nullability

int main(int argc, const char **argv) {
  using namespace clang::tooling;
  return executeFromCommandLineArgs(argc, argv, {
      clang::tidy::nullability::run,
      // Disable warnings, testcases are full of unused expressions etc.
      getInsertArgumentAdjuster("-w", ArgumentInsertPosition::BEGIN});
}
