// Define an enum named `Direction` with four variants
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::*; // Import all variants of `Direction` directly

// Function to show direction
fn show_direction(dir: Direction) {
    // Match the `dir` against each variant of `Direction`
    match dir {
        Up => println!("Going up"),    // If dir is Up
        Down => println!("Going down"),  // If dir is Down
        Left => println!("Going left"),  // If dir is Left
        Right => println!("Going right"), // If dir is Right
    }
}

fn main() {
    let dir = Right; // Use `Right` directly without `Direction::`
    show_direction(dir); 

    // More examples
    let dir2 = Up;
    show_direction(dir2);

    let dir3 = Down;
    show_direction(dir3);
}
