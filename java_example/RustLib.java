package com.example.rustlib;

/**
 * Java interface to the Rust library functions.
 * Uses JNI to call native Rust functions.
 */
public class RustLib {
    
    // Load the native library
    static {
        System.loadLibrary("jni_lib");
    }
    
    // Basic arithmetic functions
    public static native int addNumbers(int a, int b);
    public static native double multiplyDoubles(double a, double b);
    
    // Mathematical functions
    public static native int factorial(int n);
    public static native boolean isPrime(int n);
    public static native int fibonacci(int n);
    
    // String functions
    public static native int stringLength(String s);
    public static native String reverseString(String s);
    
    // Array functions
    public static native int sumArray(int[] arr);
    
    /**
     * Main method to test all the Rust library functions
     */
    public static void main(String[] args) {
        System.out.println("=== Rust Library Functions Called from Java ===");
        
        // Test basic arithmetic
        System.out.println("\n--- Basic Arithmetic ---");
        int sum = addNumbers(5, 3);
        System.out.printf("addNumbers(5, 3) = %d%n", sum);
        
        double product = multiplyDoubles(2.5, 4.0);
        System.out.printf("multiplyDoubles(2.5, 4.0) = %.2f%n", product);
        
        // Test mathematical functions
        System.out.println("\n--- Mathematical Functions ---");
        int fact = factorial(5);
        System.out.printf("factorial(5) = %d%n", fact);
        
        boolean prime = isPrime(17);
        System.out.printf("isPrime(17) = %s%n", prime);
        
        boolean prime2 = isPrime(16);
        System.out.printf("isPrime(16) = %s%n", prime2);
        
        int fib = fibonacci(10);
        System.out.printf("fibonacci(10) = %d%n", fib);
        
        // Test string functions
        System.out.println("\n--- String Functions ---");
        String testStr = "Hello, World!";
        int length = stringLength(testStr);
        System.out.printf("stringLength(\"%s\") = %d%n", testStr, length);
        
        String reversed = reverseString(testStr);
        System.out.printf("reverseString(\"%s\") = \"%s\"%n", testStr, reversed);
        
        // Test array functions
        System.out.println("\n--- Array Functions ---");
        int[] numbers = {1, 2, 3, 4, 5};
        int arraySum = sumArray(numbers);
        System.out.printf("sumArray([1, 2, 3, 4, 5]) = %d%n", arraySum);
        
        System.out.println("\n=== All tests completed successfully! ===");
    }
}
