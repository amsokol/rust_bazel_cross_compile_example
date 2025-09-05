use libc::{c_char, c_double, c_int};
use std::ffi::{CStr, CString};

/// Add two integers
/// # Safety
/// This function is safe to call from C/Go
#[unsafe(no_mangle)]
pub extern "C" fn add_numbers(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Multiply two doubles
/// # Safety
/// This function is safe to call from C/Go
#[unsafe(no_mangle)]
pub extern "C" fn multiply_doubles(a: c_double, b: c_double) -> c_double {
    a * b
}

/// Calculate factorial of a number
/// # Safety
/// This function is safe to call from C/Go
#[unsafe(no_mangle)]
pub extern "C" fn factorial(n: c_int) -> c_int {
    if n <= 1 { 1 } else { (1..=n).product() }
}

/// Check if a number is prime
/// # Safety
/// This function is safe to call from C/Go
#[unsafe(no_mangle)]
pub extern "C" fn is_prime(n: c_int) -> c_int {
    if n < 2 {
        return 0; // false
    }
    if n == 2 {
        return 1; // true
    }
    if n % 2 == 0 {
        return 0; // false
    }

    let limit = (n as f64).sqrt() as c_int;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return 0; // false
        }
    }
    1 // true
}

/// Get length of a string
/// # Safety
/// The caller must ensure that `s` is a valid null-terminated C string
#[unsafe(no_mangle)]
pub unsafe extern "C" fn string_length(s: *const c_char) -> c_int {
    if s.is_null() {
        return -1;
    }

    unsafe {
        match CStr::from_ptr(s).to_str() {
            Ok(rust_str) => rust_str.len() as c_int,
            Err(_) => -1,
        }
    }
}

/// Reverse a string (returns a new allocated string)
/// # Safety
/// The caller must ensure that `s` is a valid null-terminated C string
/// The caller is responsible for freeing the returned string using `free_string`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reverse_string(s: *const c_char) -> *mut c_char {
    if s.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        match CStr::from_ptr(s).to_str() {
            Ok(rust_str) => {
                let reversed = rust_str.chars().rev().collect::<String>();
                match CString::new(reversed) {
                    Ok(c_string) => c_string.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Free a string allocated by this library
/// # Safety
/// The caller must ensure that `s` was allocated by this library
/// and has not been freed before
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Calculate Fibonacci number (iterative approach)
/// # Safety
/// This function is safe to call from C/Go
#[unsafe(no_mangle)]
pub extern "C" fn fibonacci(n: c_int) -> c_int {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

/// Sum an array of integers
/// # Safety
/// The caller must ensure that `arr` points to a valid array of `len` integers
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum_array(arr: *const c_int, len: c_int) -> c_int {
    if arr.is_null() || len <= 0 {
        return 0;
    }

    unsafe {
        let slice = std::slice::from_raw_parts(arr, len as usize);
        slice.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(5, 3), 8);
        assert_eq!(add_numbers(-5, 10), 5);
    }

    #[test]
    fn test_multiply_doubles() {
        assert_eq!(multiply_doubles(2.5, 4.0), 10.0);
        assert_eq!(multiply_doubles(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), 1);
        assert_eq!(is_prime(17), 1);
        assert_eq!(is_prime(4), 0);
        assert_eq!(is_prime(1), 0);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_string_functions() {
        let test_str = CString::new("hello").unwrap();

        unsafe {
            assert_eq!(string_length(test_str.as_ptr()), 5);

            let reversed = reverse_string(test_str.as_ptr());
            assert!(!reversed.is_null());

            let reversed_str = CStr::from_ptr(reversed).to_str().unwrap();
            assert_eq!(reversed_str, "olleh");

            free_string(reversed);
        }
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        unsafe {
            assert_eq!(sum_array(arr.as_ptr(), 5), 15);
        }
    }
}
