fn main() {
    // Initialize a variable to start counting from
    let mut count = 1;

    // Start a loop that runs until count reaches 20
    while count <= 20 {
        // Check if the current number is even
        if count % 2 == 0 {
            // Print the current even number
            println!("Even number: {}", count);
        }

        // Increment count by 1
        count += 1;
    }
}
