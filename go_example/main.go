package main

/*
#include "go_lib.h"
#include <stdlib.h>
*/
import "C"

import (
	"fmt"
	"unsafe"
)

func main() {
	fmt.Println("=== Rust Library Functions Called from Go ===")

	// Test basic arithmetic
	fmt.Println("\n--- Basic Arithmetic ---")
	sum := C.add_numbers(5, 3)
	fmt.Printf("add_numbers(5, 3) = %d\n", int(sum))

	product := C.multiply_doubles(2.5, 4.0)
	fmt.Printf("multiply_doubles(2.5, 4.0) = %.2f\n", float64(product))

	// Test mathematical functions
	fmt.Println("\n--- Mathematical Functions ---")
	fact := C.factorial(5)
	fmt.Printf("factorial(5) = %d\n", int(fact))

	prime := C.is_prime(17)
	fmt.Printf("is_prime(17) = %s\n", boolToString(int(prime)))

	prime2 := C.is_prime(16)
	fmt.Printf("is_prime(16) = %s\n", boolToString(int(prime2)))

	fib := C.fibonacci(10)
	fmt.Printf("fibonacci(10) = %d\n", int(fib))

	// Test string functions
	fmt.Println("\n--- String Functions ---")
	testStr := "Hello, World!"
	cStr := C.CString(testStr)
	defer C.free(unsafe.Pointer(cStr))

	length := C.string_length(cStr)
	fmt.Printf("string_length(\"%s\") = %d\n", testStr, int(length))

	reversedPtr := C.reverse_string(cStr)
	if reversedPtr != nil {
		reversed := C.GoString(reversedPtr)
		fmt.Printf("reverse_string(\"%s\") = \"%s\"\n", testStr, reversed)
		C.free_string(reversedPtr)
	}

	// Test array functions
	fmt.Println("\n--- Array Functions ---")
	numbers := []C.int{1, 2, 3, 4, 5} // Use C.int instead of Go int
	cArray := &numbers[0]
	arraySum := C.sum_array(cArray, C.int(len(numbers)))
	fmt.Printf("sum_array([1, 2, 3, 4, 5]) = %d\n", int(arraySum))

	fmt.Println("\n=== All tests completed successfully! ===")
}

func boolToString(val int) string {
	if val != 0 {
		return "true"
	}
	return "false"
}
