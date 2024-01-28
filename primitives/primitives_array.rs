use std::mem;

// Analyze and print information about a slice of `i32`.
fn analyze_slice(slice: &[i32]) {
    if !slice.is_empty() {
        println!("First element of the slice: {}", slice[0]);
    }
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // A fixed-size array of `i32` with 5 elements.
    let fixed_size_array: [i32; 5] = [1, 2, 3, 4, 5];

    // An array of 500 elements; all initialized to 0.
    let array_500_elements: [i32; 500] = [0; 500];

    // Demonstrating simple array indexing (0-based).
    println!("First element of the array: {}", fixed_size_array[0]);
    println!("Second element of the array: {}", fixed_size_array[1]);

    // Using `len` to find the number of elements in the array.
    println!("Number of elements in array: {}", fixed_size_array.len());

    // Showing the memory size of the array.
    println!("Array occupies {} bytes", mem::size_of_val(&fixed_size_array));

    // Borrowing the entire array as a slice and analyzing it.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&fixed_size_array);

    // Borrowing a subsection of the array as a slice.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&array_500_elements[1..4]);

    // Working with an empty array and comparing it to empty slices.
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // More verbose comparison with an empty slice

    // Safely accessing array elements using `.get` which returns an Option.
    // If out of bounds, `None` is returned, preventing a crash.
    for i in 0..fixed_size_array.len() + 1 {
        match fixed_size_array.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! Index {} is out of bounds!", i),
        }
    }

    // Uncommenting the below line will result in a compile-time error due to out of bound indexing.
    // println!("{}", fixed_size_array[5]);

    // Out of bound indexing on slices will cause a runtime panic.
    // Uncommenting the below line will result in a runtime error.
    // println!("{}", &fixed_size_array[..][5]);
}
