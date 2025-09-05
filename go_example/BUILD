load("@rules_go//go:def.bzl", "go_binary", "go_cross_binary")
load("//:settings.bzl", "PLATFORMS_PER_ARCH")

ARCHS = [
    "arm64",
    "amd64",
]

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
    linkmode = "pie",
    pure = "off",
)

[
    # Use gcr.io/distroless/cc-debian12
    # To run the binary in OCI container.
    go_cross_binary(
        name = "go_example_%s" % arch,
        platform = PLATFORMS_PER_ARCH[arch],
        target = "go_example",
    )
    for arch in ARCHS
]
