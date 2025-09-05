# Rust Library for Go Integration

This directory contains a Rust library (`go_lib`) that exposes C-compatible functions which can be called from Go applications.

## Library Features

The Rust library provides the following functions:

### Basic Arithmetic
- `add_numbers(a, b)` - Adds two integers
- `multiply_doubles(a, b)` - Multiplies two double-precision floating-point numbers

### Mathematical Functions
- `factorial(n)` - Calculates the factorial of a number
- `is_prime(n)` - Checks if a number is prime (returns 1 for true, 0 for false)
- `fibonacci(n)` - Calculates the nth Fibonacci number

### String Functions
- `string_length(s)` - Returns the length of a C string
- `reverse_string(s)` - Returns a new string with characters in reverse order
- `free_string(s)` - Frees a string allocated by the library

### Array Functions
- `sum_array(arr, len)` - Sums all elements in an integer array

## Building the Library

### Using Cargo (Standard Rust Build)

1. Build the library as a shared library (`.so` on Linux, `.dylib` on macOS, `.dll` on Windows):
   ```bash
   cd rust/go_lib
   cargo build --release
   ```

2. The compiled library will be located at:
   ```
   target/release/libgo_lib.so    # Linux
   target/release/libgo_lib.dylib # macOS
   target/release/go_lib.dll      # Windows
   ```

### Using Bazel

1. Build the dynamic library:
   ```bash
   bazel build //rust/go_lib:go_lib
   ```

2. Build the static library:
   ```bash
   bazel build //rust/go_lib:go_lib_static
   ```

3. Run the tests:
   ```bash
   bazel test //rust/go_lib:go_lib_test
   ```

## Using from Go

### Prerequisites
- Go 1.21 or later
- The compiled Rust library (`.so`, `.dylib`, or `.dll`)
- The C header file (`go_lib.h`)

### Building and Running the Go Example

1. First, build the Rust library:
   ```bash
   cd rust/go_lib
   cargo build --release
   ```

2. Then build and run the Go example:
   ```bash
   cd go_example
   go run main.go
   ```

### Integration in Your Go Project

1. Copy the header file (`go_lib.h`) to your Go project
2. Copy the compiled library to your project or system library path
3. Use CGO directives in your Go code:

```go
/*
#cgo LDFLAGS: -L/path/to/library -lgo_lib -ldl -lm
#include "go_lib.h"
*/
import "C"
```

### Example Usage

```go
package main

import "C"
import "fmt"

func main() {
    // Call Rust function from Go
    result := C.add_numbers(5, 3)
    fmt.Printf("5 + 3 = %d\n", int(result))
    
    // Check if number is prime
    isPrime := C.is_prime(17)
    fmt.Printf("17 is prime: %t\n", int(isPrime) != 0)
}
```

## Memory Management

When working with string functions:
- Strings returned by `reverse_string()` must be freed using `free_string()`
- Always check for null pointers before using returned strings
- The Go example demonstrates proper memory management

## Safety Considerations

- All functions are marked as `unsafe` in Rust where appropriate
- Null pointer checks are implemented for string and array functions
- The library handles UTF-8 validation for string operations
- Error conditions return appropriate values (-1 for errors, null for invalid strings)

## Testing

Run the Rust tests:
```bash
cd rust/go_lib
cargo test
```

Run with Bazel:
```bash
bazel test //rust/go_lib:go_lib_test
```

## Cross-Compilation

The library can be cross-compiled for different targets using Rust's cross-compilation features:

```bash
# For example, to compile for ARM64 Linux:
cargo build --release --target aarch64-unknown-linux-gnu
```

Make sure you have the appropriate cross-compilation toolchain installed.
