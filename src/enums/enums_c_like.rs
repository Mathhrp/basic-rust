// Define a C-like enum for status codes
enum StatusCode {
    Success = 200,
    NotFound = 404,
    InternalServerError = 500,
}

// Function to display the status code
fn show_status_code(code: StatusCode) {
    // Cast enum variant to `i32` and print it
    println!("Status code is {}", code as i32);
}

fn main() {
    let code = StatusCode::NotFound; // Use `NotFound` variant
    show_status_code(code); 

    // More examples
    let code2 = StatusCode::Success;
    show_status_code(code2);

    let code3 = StatusCode::InternalServerError;
    show_status_code(code3);
}
