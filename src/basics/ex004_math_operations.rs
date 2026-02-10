// src/basics/ex004_math_operations.rs
pub fn math_operations() {
    let a: i32 = 10;
    let b: i32 = 5;

    // Addition
    let sum = a + b;
    println!("{} + {} = {}", a, b, sum);

    // Subtraction
    let difference = a - b;
    println!("{} - {} = {}", a, b, difference);

    // Multiplication
    let product = a * b;
    println!("{} * {} = {}", a, b, product);
    
    // Division
    if b != 0 {
        let quotient = a / b;
        println!("{} / {} = {}", a, b, quotient);
    } else {
        println!("Cannot divide by zero");
    }
}