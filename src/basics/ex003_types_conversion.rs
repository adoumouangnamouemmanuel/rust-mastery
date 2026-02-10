fn types_conversion() {
    // Convert an integer to a string
    let number: i32 = 42;
    let number_as_string = number.to_string();
    println!("The integer {} as a string is: {}", number, number_as_string);

    // Convert a string to an integer
    let string_number = "123";
    match string_number.parse::<i32>() {
        Ok(parsed_number) => println!("The string '{}' as an integer is: {}", string_number, parsed_number),
        Err(e) => println!("Failed to convert '{}' to an integer: {}", string_number, e),
    }
}