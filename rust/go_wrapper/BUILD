load("@rules_rust//rust:defs.bzl", "rust_shared_library", "rust_static_library", "rust_test")

# Export the header file for CGO
exports_files(["go_lib.h"])

version = "0.1.0"

rust_shared_library(
    name = "go_lib",
    srcs = ["src/lib.rs"],
    version = version,
    visibility = ["//visibility:public"],
    deps = [
        "//rust/core_lib",
        "@cargo//:libc",
    ],
)

rust_static_library(
    name = "go_lib_static",
    srcs = ["src/lib.rs"],
    rustc_flags = [
        "-Crelocation-model=pic",
    ],
    version = version,
    visibility = ["//visibility:public"],
    deps = [
        "//rust/core_lib",
        "@cargo//:libc",
    ],
)

rust_test(
    name = "go_wrapper_test",
    crate = ":go_lib",
)
