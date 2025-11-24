pub fn print_message(message: String) {
    // Done: Implement the function
    // Should print `Message: <message>`
    println!("Message: {}", message);
}

fn main() {
    print_message("Hello world".to_string());
}
