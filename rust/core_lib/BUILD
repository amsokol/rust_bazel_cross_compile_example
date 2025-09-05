load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "core_lib",
    srcs = ["src/lib.rs"],
    visibility = ["//visibility:public"],
)

rust_test(
    name = "core_lib_test",
    crate = ":core_lib",
)
