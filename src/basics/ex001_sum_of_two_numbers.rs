// Reads two numbers from stdin and prints their sum.
pub fn sum_of_two_numbers() {
    
    // Prompt for the first number
    println!("Enter the first number:");
    let mut num1 = String::new();
    std::io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    // Parse the first number as f64
    let num1: f64 = num1
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Prompt for the second number
    println!("Enter the second number:");
    let mut num2 = String::new();
    std::io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    // Parse the second number as f64
    let num2: f64 = num2
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Compute and display the sum
    let sum = num1 + num2;
    println!("The sum of {} and {} is {}", num1, num2, sum);
}