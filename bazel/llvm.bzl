# Part of the Crubit project, under the Apache License v2.0 with LLVM
# Exceptions. See /LICENSE for license information.
# SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

# Create a loader/trampoline repository that we can call into to load LLVM.
#
# Our real goal is to choose between two different sources for LLVM binaries:
#  - if `PREBUILT_LLVM_PATH` is in the environment, we treat it as the root of
#    an LLVM tree that's been built with CMake and try to use headers and
#    libraries from there
#  - otherwise, we build LLVM from source
#
# We *could* implement this choice directly as an if/else between `http_archive`
# or `new_local_repository`. However, all Bazel `load`s are unconditional, so we
# would always end up cloning the (very large) LLVM project repository to load
# its Bazel configuration even if we aren't going to use it.
#
# To avoid that, we add the extra indirection of the "loader" repository. We
# populate the loader repository with one of two templated .bzl files depending
# on whether we want "local" or "remote" LLVM. Then our caller activates that
# .bzl file and gets the desired effect.
def _llvm_loader_repository(repository_ctx):
    # The final repository is required to have a `BUILD` file at the root.
    repository_ctx.file("BUILD")

    # Create `llvm.bzl` from one of `llvm_{remote|local}.bzl.tmpl`.
    if "PREBUILT_LLVM_PATH" in repository_ctx.os.environ:
        # Use prebuilt LLVM
        path = repository_ctx.os.environ["PREBUILT_LLVM_PATH"]

        # If needed, resolve relative to root of *calling* repository
        if not path.startswith("/"):
            root_path = repository_ctx.path(
                repository_ctx.attr.file_at_root,
            ).dirname
            path = repository_ctx.path(str(root_path) + "/" + path)

        repository_ctx.template(
            "llvm.bzl",
            Label("//bazel:llvm_local.bzl.tmpl"),
            substitutions = {
                "${PREBUILT_LLVM_PATH}": str(path),
                "${CMAKE_BUILD_DIR}": "build",
            },
            executable = False,
        )
    else:
        # Use downloaded LLVM built with Bazel
        repository_ctx.template(
            "llvm.bzl",
            Label("//bazel:llvm_remote.bzl.tmpl"),
            substitutions = {},
            executable = False,
        )

def llvm_loader_repository_dependencies():
    # This *declares* the dependency, but it won't actually be *downloaded*
    # unless it's used.
    http_archive(
        name = "llvm-raw",
        build_file_content = "# empty",
        sha256 = "eb7437b60a6f78e7910d08911975f100e99e9c714f069a5487119c7eadc79171",
        strip_prefix = "llvm-project-llvmorg-14.0.0",
        urls = ["https://github.com/llvm/llvm-project/archive/refs/tags/llvmorg-14.0.0.zip"],
    )

llvm_loader_repository = repository_rule(
    implementation = _llvm_loader_repository,
    attrs = {
        # We need a file from the root in order to get the workspace path
        "file_at_root": attr.label(default = "//:BUILD"),
    },
    environ = [
        "PREBUILT_LLVM_PATH",
    ],
)