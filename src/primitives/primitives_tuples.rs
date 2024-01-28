// Demonstrates how tuples can be used in Rust, including in functions.

// A function that takes a tuple as an argument and returns a tuple.
// This function reverses the elements of the input tuple.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // Destructuring the tuple into individual variables.
    let (int_param, bool_param) = pair;

    // Returning a new tuple with elements in reversed order.
    (bool_param, int_param)
}

// A struct named `Matrix` with four fields of type `f32`.
// The `derive(Debug)` annotation enables printing of the struct for debugging.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // Creating a tuple with various types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // Accessing elements of a tuple using index-based notation.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can contain other tuples as elements.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Printing a tuple. This is possible because of the `Debug` trait.
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // Note: Tuples with more than 12 elements cannot be printed directly.
    // Uncommenting the below lines will result in a compiler error.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    // A simple tuple of an integer and a boolean.
    let pair = (1, true);
    println!("Pair is {:?}", pair);

    // Using the `reverse` function to reverse the elements of the tuple.
    println!("The reversed pair is {:?}", reverse(pair));

    // Demonstrating how to create a single-element tuple.
    // The comma is necessary to differentiate it from just a number in parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32)); // This is not a tuple, just an integer.

    // Destructuring a tuple into separate variables.
    let tuple = (1, "hello", 4.5, true);
    let (first, second, third, fourth) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", first, second, third, fourth);

    // Creating an instance of the `Matrix` struct and printing it.
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
