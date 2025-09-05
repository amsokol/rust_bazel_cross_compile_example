/// Core mathematical and utility functions
/// This library contains the core functionality that can be used by various wrappers

/// Add two integers
pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two doubles
pub fn multiply_doubles(a: f64, b: f64) -> f64 {
    a * b
}

/// Calculate factorial of a number
pub fn factorial(n: i32) -> i32 {
    if n <= 1 { 1 } else { (1..=n).product() }
}

/// Check if a number is prime
pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

/// Calculate Fibonacci number (iterative approach)
pub fn fibonacci(n: i32) -> i32 {
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

/// Get length of a string
pub fn string_length(s: &str) -> i32 {
    s.len() as i32
}

/// Reverse a string
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// Sum an array of integers
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(4), false);
        assert_eq!(is_prime(1), false);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_string_functions() {
        assert_eq!(string_length("hello"), 5);
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_sum_array() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(sum_array(&arr), 15);
    }
}
