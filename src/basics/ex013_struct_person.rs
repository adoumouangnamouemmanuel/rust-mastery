// Define a struct named 'Person' with fields 'name' and 'age'
struct Person {
    name: String,
    age: u32,
}

// main function
fn main() {
    // Create a new instance of 'Person' struct
    let person1 = Person {
        // Assign values to 'name' and 'age' fields
        name: String::from("Salih Yejide"),
        age: 30,
    };

    // Access and print the fields of 'person1'
    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
}
