load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//:settings.bzl", "RUST_BUILD_FLAGS_PER_ARCH", "RUST_PLATFORMS_PER_ARCH")

version = "0.1.0"

rust_binary(
    name = "hello_world_arm64",
    srcs = [
        "src/main.rs",
    ],
    platform = RUST_PLATFORMS_PER_ARCH["arm64"],
    rustc_flags = RUST_BUILD_FLAGS_PER_ARCH["arm64"],
    version = version,
    deps = [
        # "@cargo//:mimalloc",
        "@cargo//:tokio",
    ],
)

rust_binary(
    name = "hello_world_amd64",
    srcs = [
        "src/main.rs",
    ],
    platform = RUST_PLATFORMS_PER_ARCH["amd64"],
    rustc_flags = RUST_BUILD_FLAGS_PER_ARCH["amd64"],
    version = version,
    deps = [
        # "@cargo//:mimalloc",
        "@cargo//:tokio",
    ],
)
