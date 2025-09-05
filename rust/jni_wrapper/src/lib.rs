/// JNI wrapper for the core library
/// This module provides JNI-compatible functions that can be called from Java
use core_lib;
use jni::objects::{JClass, JIntArray, JString};
use jni::sys::{jboolean, jdouble, jint};
use jni::JNIEnv;

// Basic arithmetic functions
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_addNumbers(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    core_lib::add_numbers(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_multiplyDoubles(
    _env: JNIEnv,
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    core_lib::multiply_doubles(a, b)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_factorial(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jint {
    core_lib::factorial(n)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_isPrime(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jboolean {
    if core_lib::is_prime(n) {
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_fibonacci(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jint {
    core_lib::fibonacci(n)
}

// String functions
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_stringLength<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> jint {
    let input_str: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    core_lib::string_length(&input_str)
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_reverseString<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    let input_str: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    let reversed = core_lib::reverse_string(&input_str);

    env.new_string(reversed)
        .expect("Couldn't create java string!")
}

// Array functions
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_sumArray<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JIntArray<'local>,
) -> jint {
    let array_length = env.get_array_length(&input).unwrap() as usize;
    let mut elements = vec![0i32; array_length];

    env.get_int_array_region(&input, 0, &mut elements)
        .expect("Couldn't get java int array elements!");

    core_lib::sum_array(&elements)
}
