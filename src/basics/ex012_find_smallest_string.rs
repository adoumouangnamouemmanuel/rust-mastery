// Define a function named 'find_smallest' that takes two string references 'str1' and 'str2'
fn find_smallest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // Check if 'str1' is less than 'str2' lexicographically
    // If it is, return 'str1', otherwise return 'str2'
    if str1 < str2 {
        str1
    } else {
        str2
    }
}