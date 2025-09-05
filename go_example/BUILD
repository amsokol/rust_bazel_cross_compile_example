load("@rules_go//go:def.bzl", "go_binary", "go_cross_binary")
load("//:settings.bzl", "PLATFORMS_PER_ARCH")

# Default Go binary (with CGO and Rust integration)
go_binary(
    name = "go_example",
    srcs = [
        "go_lib.h",
        "main.go",
    ],
    cdeps = [
        "//rust/go_lib:go_lib",
    ],
    cgo = True,
    clinkopts = [
        "-fPIC",
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
    pure = "off",
)

ARCHS = [
    "arm64",
    "amd64",
]

[
    go_cross_binary(
        name = "go_example_%s" % arch,
        platform = PLATFORMS_PER_ARCH[arch],
        target = "go_example",
    )
    for arch in ARCHS
]
