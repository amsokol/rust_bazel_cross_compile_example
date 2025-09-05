load("@rules_java//java:defs.bzl", "java_binary", "java_library")

java_library(
    name = "rust_lib",
    srcs = ["RustLib.java"],
    visibility = ["//visibility:public"],
)

java_binary(
    name = "java_example",
    data = ["//rust/jni_lib:go_lib"],
    jvm_flags = [
        "-Djava.library.path=rust/jni_lib",
    ],
    main_class = "com.example.rustlib.RustLib",
    runtime_deps = [":rust_lib"],
)
