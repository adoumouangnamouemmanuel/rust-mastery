// Define a function named 'factorial' that takes a parameter 'n' of type u64 and returns a u64
fn factorial(n: u64) -> u64 {
    // Initialize the result variable with 1
    let mut result = 1;

    // Start a for loop to calculate the factorial
    for i in 1..=n {
        // Multiply the result by the current value of 'i'
        result *= i;
    }

    // Return the calculated factorial
    result
}