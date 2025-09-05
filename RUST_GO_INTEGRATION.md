# Rust Library for Go Integration - Quick Start

This example demonstrates how to create a Rust library with C-compatible functions that can be called from Go applications.

## What's Included

- **Rust Library** (`rust/go_lib/`): A Rust library with mathematical, string, and array functions
- **Go Example** (`go_example/`): A complete example showing how to call Rust functions from Go
- **C Header** (`rust/go_lib/go_lib.h`): C interface definitions for Go integration
- **Build Script** (`build_and_run.sh`): Automated build and test script

## Quick Start

1. **Build and Run Everything**:
   ```bash
   ./build_and_run.sh
   ```

2. **Manual Steps**:
   ```bash
   # Build the Rust library
   cargo build --release -p go_lib
   
   # Run Rust tests
   cargo test -p go_lib
   
   # Run the Go example
   cd go_example
   LD_LIBRARY_PATH=../target/release go run main.go
   ```

## Available Functions

### Basic Arithmetic
- `add_numbers(a, b)` - Adds two integers
- `multiply_doubles(a, b)` - Multiplies two doubles

### Mathematical Functions  
- `factorial(n)` - Calculates factorial
- `is_prime(n)` - Checks if number is prime
- `fibonacci(n)` - Calculates nth Fibonacci number

### String Functions
- `string_length(s)` - Returns string length
- `reverse_string(s)` - Returns reversed string (must free with `free_string`)
- `free_string(s)` - Frees string allocated by library

### Array Functions
- `sum_array(arr, len)` - Sums array elements

## Key Implementation Notes

### Memory Management
- Strings returned by `reverse_string()` must be freed using `free_string()`
- Always check for null pointers when working with strings

### Type Compatibility
- Use `C.int` instead of Go's `int` for arrays (Go int is 64-bit, C int is 32-bit)
- Example:
  ```go
  numbers := []C.int{1, 2, 3, 4, 5}  // Correct
  numbers := []int{1, 2, 3, 4, 5}    // Wrong - causes memory issues
  ```

### CGO Integration
```go
/*
#cgo LDFLAGS: -L../target/release -lgo_lib -ldl -lm
#include "../rust/go_lib/go_lib.h"
*/
import "C"
```

## Files Created

```
rust/go_lib/
├── Cargo.toml          # Rust library configuration
├── BUILD               # Bazel build file  
├── go_lib.h           # C header for Go integration
├── README.md          # Detailed documentation
└── src/
    └── lib.rs         # Rust implementation with FFI functions

go_example/
├── go.mod             # Go module file
└── main.go            # Complete Go example

build_and_run.sh       # Automated build script
```

## Example Output

```
=== Rust Library Functions Called from Go ===

--- Basic Arithmetic ---
add_numbers(5, 3) = 8
multiply_doubles(2.5, 4.0) = 10.00

--- Mathematical Functions ---
factorial(5) = 120
is_prime(17) = true
is_prime(16) = false
fibonacci(10) = 55

--- String Functions ---
string_length("Hello, World!") = 13
reverse_string("Hello, World!") = "!dlroW ,olleH"

--- Array Functions ---
sum_array([1, 2, 3, 4, 5]) = 15

=== All tests completed successfully! ===
```

## Next Steps

1. **Add More Functions**: Extend `rust/go_lib/src/lib.rs` with additional functionality
2. **Cross-Compilation**: Use Rust's cross-compilation for different target platforms
3. **Error Handling**: Implement more sophisticated error handling patterns
4. **Performance**: Benchmark and optimize critical functions
5. **Packaging**: Create proper Go modules and Rust crates for distribution

## Safety and Best Practices

- All FFI functions are properly marked as `unsafe` where required
- Null pointer checks are implemented for all pointer operations
- Memory allocated by Rust is properly freed through dedicated functions
- UTF-8 validation is handled for string operations
- Comprehensive tests ensure reliability

For detailed documentation, see `rust/go_lib/README.md`.
