// Define an enum named `BasicColor` with three variants
enum BasicColor {
    Red,
    Green,
    Blue,
}

// Function to display the color
fn show_basic_color(color: BasicColor) {
    // Match the `color` against each variant of `BasicColor`
    match color {
        BasicColor::Red => println!("Red"),   // If color is Red
        BasicColor::Green => println!("Green"), // If color is Green
        BasicColor::Blue => println!("Blue"),  // If color is Blue
    }
}

fn main() {
    let color = BasicColor::Red; // Create a `BasicColor` instance with `Red` variant
    show_basic_color(color);     // Call function to display the color

    // More examples
    let color2 = BasicColor::Green; // Use the Green variant
    show_basic_color(color2);

    let color3 = BasicColor::Blue; // Use the Blue variant
    show_basic_color(color3);
}
