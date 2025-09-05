#ifndef GO_LIB_H
#define GO_LIB_H

#ifdef __cplusplus
extern "C" {
#endif

// Basic arithmetic functions
int add_numbers(int a, int b);
double multiply_doubles(double a, double b);

// Mathematical functions
int factorial(int n);
int is_prime(int n);
int fibonacci(int n);

// String functions
int string_length(const char* s);
char* reverse_string(const char* s);
void free_string(char* s);

// Array functions
int sum_array(const int* arr, int len);

#ifdef __cplusplus
}
#endif

#endif // GO_LIB_H
