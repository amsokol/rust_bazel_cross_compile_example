load("@rules_java//java:defs.bzl", "java_binary", "java_library")

ARCHS = [
    "arm64",
    "amd64",
]

java_library(
    name = "rust_lib",
    srcs = ["RustLib.java"],
    visibility = ["//visibility:public"],
)

java_binary(
    name = "java_example",
    data = ["//rust/jni_wrapper:jni_lib"],
    jvm_flags = [
        "-Djava.library.path=rust/jni_wrapper",
    ],
    main_class = "com.example.rustlib.RustLib",
    runtime_deps = [":rust_lib"],
)

[
    java_binary(
        name = "java_example_%s" % arch,
        data = ["//rust/jni_wrapper:jni_lib_%s" % arch],
        jvm_flags = [
            "-Djava.library.path=rust/jni_wrapper_%s" % arch,
        ],
        main_class = "com.example.rustlib.RustLib",
        runtime_deps = [":rust_lib"],
    )
    for arch in ARCHS
]
