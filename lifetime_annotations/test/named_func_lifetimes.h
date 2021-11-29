// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_
#define CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_

#include <initializer_list>
#include <iostream>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "lifetime_annotations/function_lifetimes.h"
#include "testing/base/public/gunit.h"
#include "third_party/llvm/llvm-project/llvm/include/llvm/ADT/StringMap.h"

namespace devtools_rust {

// Returns a human-readable representation of `func_lifetimes` that uses
// alphabetic names for lifetimes.
std::string NameLifetimes(const FunctionLifetimes& func_lifetimes);

// Associates functions (identified by their name) with function lifetimes in
// the format returned by NameLifetimes().
class NamedFuncLifetimes {
 public:
  NamedFuncLifetimes() = default;

  NamedFuncLifetimes(const NamedFuncLifetimes&) = default;
  NamedFuncLifetimes(NamedFuncLifetimes&&) = default;
  NamedFuncLifetimes& operator=(const NamedFuncLifetimes&) = default;
  NamedFuncLifetimes& operator=(NamedFuncLifetimes&&) = default;

  NamedFuncLifetimes(
      std::initializer_list<std::pair<llvm::StringRef, std::string>> values)
      : lifetimes_(std::move(values)) {}

  // Associates the function called `func` with the named lifetimes `lifetimes`.
  void Add(llvm::StringRef func, llvm::StringRef lifetimes) {
    lifetimes_.try_emplace(func, lifetimes);
  }

  // Returns the named lifetimes for the function called `func`.
  std::optional<llvm::StringRef> Get(llvm::StringRef func) const;

  // Returns the "function name, lifetimes" entries in the mapping, sorted
  // alphabetically.
  std::vector<std::pair<llvm::StringRef, llvm::StringRef>> Entries() const;

  bool operator==(const NamedFuncLifetimes& other) const {
    return lifetimes_ == other.lifetimes_;
  }
  bool operator!=(const NamedFuncLifetimes& other) const {
    return !(*this == other);
  }

 private:
  friend std::ostream& operator<<(std::ostream& os,
                                  const NamedFuncLifetimes& lifetimes);

  llvm::StringMap<std::string> lifetimes_;
};

// Returns a matcher that matches a NamedFuncLifetimes equal to `expected`.
testing::Matcher<NamedFuncLifetimes> LifetimesAre(NamedFuncLifetimes expected);

// Returns a matcher that matches a NamedFuncLifetimes containing all oentries
// from `expected`, and possibly more.
testing::Matcher<NamedFuncLifetimes> LifetimesContain(
    NamedFuncLifetimes expected);

}  // namespace devtools_rust

#endif  // CRUBIT_LIFETIME_ANNOTATIONS_TEST_NAMED_FUNC_LIFETIMES_H_