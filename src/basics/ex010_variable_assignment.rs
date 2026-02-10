fn variable_assignment() {
    // Declare variable 'p' and initialize it with a value
    let p = 10;

    // Declare an unused variable '_q' instead of 'q'
    let mut _q;

    // Assign the value of 'p' to '_q'
    _q = p;

    // The following line will print the value of 'p'
    println!("Value of p: {}", p);
}
