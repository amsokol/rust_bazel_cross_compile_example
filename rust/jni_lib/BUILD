load("@rules_rust//rust:defs.bzl", "rust_shared_library")

rust_shared_library(
    name = "go_lib",  # This will create libgo_lib.so for Java to load
    srcs = ["src/lib.rs"],
    visibility = ["//visibility:public"],
    deps = [
        "//rust/go_lib:go_lib_static",
        "@cargo//:jni",
    ],
)
