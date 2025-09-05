load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "go_lib",  # Keep the same name for Java compatibility
    srcs = ["src/lib.rs"],
    visibility = ["//visibility:public"],
    deps = [
        "//rust/core_lib",
        "@cargo//:jni",
    ],
)
