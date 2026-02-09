pub fn mutable_variable() {

    // Declare a mutable variable counter and initialize it with 0  
    let mut counter = 0; 
    println!("Initial value of counter: {}", counter); // Print initial value

    counter += 1; // Increase counter by 1
    println!("Value of counter after incrementing: {}", counter); // Print value after incrementing

    counter -= 1; // Decrease counter by 1
    println!("Value of counter after decrementing: {}", counter); // Print value after decrementing
}