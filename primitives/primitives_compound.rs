fn main() {
    // Declaring a boolean variable with explicit type annotation.
    let _logical: bool = true;

    // Declaring a floating-point variable with explicit type annotation.
    // This is a 64-bit float (double precision).
    let _a_float: f64 = 1.0;  // Regular annotation

    // Declaring an integer with a type suffix.
    // The type `i32` is explicitly specified with a suffix annotation.
    let _an_integer = 5i32; // Suffix annotation

    // If no type is specified, Rust will use a default.
    // Here, `_default_float` is of type `f64` (double precision float).
    let _default_float = 3.0; // `f64`
    // And `_default_integer` is of type `i32`.
    let _default_integer = 7;   // `i32`

    // Example of type inference in Rust.
    // The type of `_inferred_type` is inferred from the context where it's used.
    let mut _inferred_type = 12; // Type inferred from the context, initially `i32`.
    _inferred_type = 4294967296i64; // Later changed to `i64` in this assignment.

    // Demonstrating a mutable variable.
    // `_mutable` is declared as mutable `i32` and can be changed.
    let mut _mutable = 12; // Mutable `i32`
    _mutable = 21; // Value changed

    // This line will cause a compile-time error because the type of `_mutable` cannot be changed.
    // `_mutable` was declared as `i32` but is being assigned a boolean value.
    // Uncommenting the line below will cause a compilation error.
    // _mutable = true;

    // Variable shadowing allows us to redeclare a variable with the same name.
    // This new `mutable` variable is a boolean, distinct from the earlier `_mutable` integer.
    let _mutable = true;
}
