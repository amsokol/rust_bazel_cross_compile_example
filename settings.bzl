"""
Module of global project settings.
"""

# Rust toolchains for different architectures.
RUST_PLATFORMS_PER_ARCH = {
    "arm64": "//:linux-aarch64",
    "amd64": "//:linux-x86_64",
}

RUST_BUILD_FLAGS_PER_ARCH = {
    "arm64": select({
        "//:optimized": [
            "-Ctarget-feature=+crt-static",
            "-Clink-arg=-flto",
            "-Clink-arg=-s",
            "-Ccodegen-units=1",
            "-Cpanic=abort",
            "-Copt-level=3",
            "-Cstrip=symbols",
            "-Clink-arg=-Wl,--no-dynamic-linker",
        ],
        "//conditions:default": [
            "-Ctarget-feature=+crt-static",
            "-Copt-level=0",
            "-Clink-arg=-Wl,--no-dynamic-linker",
        ],
    }),
    "amd64": select({
        "//:optimized": [
            "-Ctarget-feature=+crt-static",
            "-Clink-arg=-flto",
            "-Clink-arg=-s",
            "-Ccodegen-units=1",
            "-Cpanic=abort",
            "-Copt-level=3",
            "-Cstrip=symbols",
            "-Clink-arg=-Wl,--no-dynamic-linker",
        ],
        "//conditions:default": [
            "-Ctarget-feature=+crt-static",
            "-Copt-level=0",
            "-Clink-arg=-Wl,--no-dynamic-linker",
        ],
    }),
}
