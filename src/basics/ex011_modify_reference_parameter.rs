// Define a function named 'modify_value' that takes a mutable reference 'value' of type i32
fn modify_value(value: &mut i32) {
    // Increment the value by 1
    *value += 1;
}