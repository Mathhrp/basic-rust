// This directive allows unused code in this scope (useful during development)
#![allow(dead_code)]

// A struct representing a person with debug print capability
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// A unit struct with no fields
struct Unit;

// A tuple struct with two fields: an integer and a float
struct Pair(i32, f32);

// A struct representing a point in 2D space
struct Point {
    x: f32,
    y: f32,
}

// A struct representing a rectangle using two points: top left and bottom right
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Initialize a `Person` using field init shorthand
    let first_name = String::from("Peter");
    let last_name = String::from("Parker");
    let age = 27;
    let peter = Person { first_name, last_name, age };

    // Print the `Person` struct using debug formatting
    println!("{:?}", peter);

    // Create a `Point` instance
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Print point coordinates
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Create another `Point`, reusing `point.y` field
    let bottom_right = Point { x: 5.2, ..point };

    // Print second point coordinates
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure `point` into separate variables
    let Point { x: left_edge, y: top_edge } = point;

    // Create a `Rectangle` instance
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right,
    };

    // Instantiate a Unit struct
    let _unit = Unit;

    // Create a `Pair` tuple struct
    let pair = Pair(1, 0.1);

    // Print the contents of `pair`
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure `pair` into separate variables
    let Pair(integer, decimal) = pair;

    // Print the destructured values
    println!("pair contains {:?} and {:?}", integer, decimal);
}
