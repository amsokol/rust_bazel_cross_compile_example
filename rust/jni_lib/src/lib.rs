use jni::objects::{JClass, JIntArray, JString};
use jni::sys::{jboolean, jdouble, jint};
use jni::JNIEnv;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};

// External C functions from go_lib
unsafe extern "C" {
    fn add_numbers(a: c_int, b: c_int) -> c_int;
    fn multiply_doubles(a: c_double, b: c_double) -> c_double;
    fn factorial(n: c_int) -> c_int;
    fn is_prime(n: c_int) -> c_int;
    fn fibonacci(n: c_int) -> c_int;
    fn string_length(s: *const c_char) -> c_int;
    fn reverse_string(s: *const c_char) -> *mut c_char;
    fn free_string(s: *mut c_char);
    fn sum_array(arr: *const c_int, len: c_int) -> c_int;
}

// Basic arithmetic functions
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_addNumbers(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    unsafe { add_numbers(a, b) }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_multiplyDoubles(
    _env: JNIEnv,
    _class: JClass,
    a: jdouble,
    b: jdouble,
) -> jdouble {
    unsafe { multiply_doubles(a, b) }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_factorial(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jint {
    unsafe { factorial(n) }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_isPrime(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jboolean {
    unsafe {
        if is_prime(n) == 1 {
            1
        } else {
            0
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_fibonacci(
    _env: JNIEnv,
    _class: JClass,
    n: jint,
) -> jint {
    unsafe { fibonacci(n) }
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
    let c_string = CString::new(input_str).expect("Couldn't create CString");

    unsafe { string_length(c_string.as_ptr()) }
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
    let c_string = CString::new(input_str).expect("Couldn't create CString");

    unsafe {
        let reversed_ptr = reverse_string(c_string.as_ptr());
        if reversed_ptr.is_null() {
            return env
                .new_string("")
                .expect("Couldn't create empty java string!");
        }

        let reversed_cstr = CStr::from_ptr(reversed_ptr);
        let reversed_str = reversed_cstr.to_str().expect("Couldn't convert to string");
        let output = env
            .new_string(reversed_str)
            .expect("Couldn't create java string!");

        // Free the C string allocated by go_lib
        free_string(reversed_ptr);

        output
    }
}

// Array functions
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_example_rustlib_RustLib_sumArray<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JIntArray<'local>,
) -> jint {
    let array_length = env.get_array_length(&input).unwrap() as usize;
    let mut elements = vec![0i32; array_length];

    env.get_int_array_region(&input, 0, &mut elements)
        .expect("Couldn't get java int array elements!");

    unsafe { sum_array(elements.as_ptr(), elements.len() as c_int) }
}
