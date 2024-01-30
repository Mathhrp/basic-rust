// Define an enum named `List` representing a singly-linked list.
enum List {
    // `Cons` variant represents an element in the list. It stores two values:
    // - An `i32`, which is the data of the current node.
    // - A `Box<List>`, which is a smart pointer to the next node in the list.
    Cons(i32, Box<List>),
    // `Nil` variant represents the end of the list. It's used to signify that there are no more nodes.
    Nil,
}

use List::*; // Import all variants of `List` directly into the current scope for easy access.

fn main() {
    // Create a linked list with three elements: 1, 2, and 3.
    // The structure of this list is as follows:
    // Cons(1, Box pointing to Cons(2, Box pointing to Cons(3, Box pointing to Nil)))
    // Here, each `Cons` contains a value and a box. The box points to the next `Cons` in the list.
    // The last `Cons` points to `Nil`, indicating the end of the list.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // To traverse this list, you would typically use a recursive function that matches on each element.
    // Such a function would handle the `Cons` case by processing the current element and then recursively
    // calling itself with the next element, and the `Nil` case to end the recursion.
    // However, this example focuses on the structure of the list rather than traversal algorithms.
}
