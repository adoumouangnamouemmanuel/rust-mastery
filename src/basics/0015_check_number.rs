// Define a function named 'check_number' that takes an integer 'num' as input and returns a Result
fn check_number(num: i32) -> Result<i32, &'static str> {
    // Check if 'num' is positive
    if num >= 0 {
        // If 'num' is positive, return it as a success value
        Ok(num)
    } else {
        // If 'num' is negative, return an error value
        Err("Negative number encountered")
    }
}

fn main() {
    // Call the 'check_number' function with positive and negative numbers
    match check_number(5) {
        Ok(value) => println!("Success: {}", value), // Prints: Success: 5
        Err(error) => println!("Error: {}", error),   // This line won't be executed
    }

    match check_number(-3) {
        Ok(value) => println!("Success: {}", value), // This line won't be executed
        Err(error) => println!("Error: {}", error),  // Prints: Error: Negative number encountered
    }
}
