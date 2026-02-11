// Define an enum named 'Color' representing different colors
#[derive(Debug)] // Add #[derive(Debug)] to automatically implement the Debug trait
enum Color {
    // Variant representing the color Red
    Red,
    // Variant representing the color Green
    Green,
    // Variant representing the color Blue
    Blue,
}

fn main() {
    // Create variables of type 'Color' using the enum variants
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    // Print the values of the color variables
    println!("Red: {:?}", red);
    println!("Green: {:?}", green);
    println!("Blue: {:?}", blue);
}
