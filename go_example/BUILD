load("@rules_go//go:def.bzl", "go_binary", "go_cross_binary")
load("//:settings.bzl", "PLATFORMS_PER_ARCH")

# Go binary for ARM64 - uses PIE linkmode for cross-compilation compatibility
go_binary(
    name = "go_example_arm64_bin",
    srcs = [
        "go_lib.h",
        "main.go",
    ],
    cdeps = [
        "//rust/go_wrapper:go_lib_static",
    ],
    cgo = True,
    clinkopts = [
        "-fPIC",
        #        "-static-libgcc",
    ],
    copts = [
        "-fPIC",
    ],
    cppopts = [
        "-fPIC",
    ],
    cxxopts = [
        "-fPIC",
    ],
    linkmode = "pie",
    pure = "off",
)

# Go binary for AMD64 - special configuration to handle PIC requirements
go_binary(
    name = "go_example_amd64_bin",
    srcs = [
        "go_lib.h",
        "main.go",
    ],
    cdeps = [
        "//rust/go_wrapper:go_lib_static",
    ],
    cgo = True,
    clinkopts = [
        "-fPIC",
        #        "-static-libgcc",
        #        "-Wl,-z,relro,-z,now",
    ],
    copts = [
        "-fPIC",
    ],
    cppopts = [
        "-fPIC",
    ],
    cxxopts = [
        "-fPIC",
    ],
    linkmode = "pie",
    pure = "off",
)

# Default Go binary - chooses based on current platform
go_binary(
    name = "go_example",
    srcs = [
        "go_lib.h",
        "main.go",
    ],
    cdeps = [
        "//rust/go_wrapper:go_lib_static",
    ],
    cgo = True,
    clinkopts = [
        "-fPIC",
        #        "-static-libgcc",
    ],
    copts = [
        "-fPIC",
    ],
    cppopts = [
        "-fPIC",
    ],
    cxxopts = [
        "-fPIC",
    ],
    linkmode = "normal",
    pure = "off",
)

# Cross-compilation targets that use the appropriate binary for each architecture
go_cross_binary(
    name = "go_example_arm64",
    platform = PLATFORMS_PER_ARCH["arm64"],
    target = "go_example_arm64_bin",
)

go_cross_binary(
    name = "go_example_amd64",
    platform = PLATFORMS_PER_ARCH["amd64"],
    target = "go_example_amd64_bin",
)
