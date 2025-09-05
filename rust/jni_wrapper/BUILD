load("@rules_rust//rust:defs.bzl", "rust_shared_library")
load("//:settings.bzl", "PLATFORMS_PER_ARCH", "RUST_BUILD_FLAGS_PER_ARCH")

version = "0.1.0"

ARCHS = [
    "arm64",
    "amd64",
]

rust_shared_library(
    name = "jni_lib",  # Keep the same name for Java compatibility
    srcs = ["src/lib.rs"],
    version = version,
    visibility = ["//visibility:public"],
    deps = [
        "//rust/core_lib",
        "@cargo//:jni",
    ],
)

[
    rust_shared_library(
        name = "jni_lib_%s" % arch,
        srcs = ["src/lib.rs"],
        platform = PLATFORMS_PER_ARCH[arch],
        rustc_flags = RUST_BUILD_FLAGS_PER_ARCH[arch],
        version = version,
        visibility = ["//visibility:public"],
        deps = [
            "//rust/core_lib",
            "@cargo//:jni",
        ],
    )
    for arch in ARCHS
]
