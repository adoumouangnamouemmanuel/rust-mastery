fn odd_even() {
    // Prompt the user to enter a number
    println!("Enter a number:");

    // Create a mutable string variable to store user input
    let mut input = String::new();

    // Read the user input from the standard input (keyboard)
    std::io::stdin().read_line(&mut input).expect("Failed to read input.");

    // Convert the user input to an integer
    let number: i32 = input.trim().parse().expect("Please enter a valid number.");

    // Check if the number is even or odd
    if number % 2 == 0 {
        // If the number is even, print a message indicating it
        println!("The number {} is even.", number);
    } else {
        // If the number is odd, print a message indicating it
        println!("The number {} is odd.", number);
    }
}
