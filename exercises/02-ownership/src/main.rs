// Exercise 02: Ownership and Borrowing
//
// This exercise focuses on Rust's unique ownership system

fn main() {
    println!("=== Exercise 1: Understanding Ownership ===");
    exercise_1();

    println!("\n=== Exercise 2: Borrowing ===");
    exercise_2();

    println!("\n=== Exercise 3: Mutable References ===");
    exercise_3();

    println!("\n=== Exercise 4: String vs &str ===");
    exercise_4();
}

// Exercise 1: Fix the ownership errors
fn exercise_1() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // TODO: Fix this code so both println! statements work
    // Hint: Use .clone() or restructure the code
    // println!("s1: {}", s1); // This will cause an error!
    // println!("s2: {}", s2);
}

// Exercise 2: Implement a function that borrows a string
fn exercise_2() {
    let text = String::from("Rust is awesome");

    // TODO: Call print_length function (you need to implement it below)
    // print_length(...);

    // This should still work after calling print_length
    println!("Text is still available: {}", text);
}

// TODO: Implement this function to print the length of a string
// It should borrow the string, not take ownership
// fn print_length(...) {
//     println!("Length: {}", ...);
// }

// Exercise 3: Implement a function that modifies a string
fn exercise_3() {
    let mut message = String::from("Hello");

    // TODO: Call add_exclamation function (you need to implement it below)
    // add_exclamation(...);

    println!("Modified message: {}", message); // Should print "Hello!!!"
}

// TODO: Implement this function to add "!!!" to the end of a string
// It should take a mutable reference
// fn add_exclamation(...) {
//     ...
// }

// Exercise 4: Work with both String and &str
fn exercise_4() {
    let string = String::from("owned string");
    let str_slice = "string slice";

    // TODO: Implement a function that accepts both String and &str
    // Hint: Use &str as the parameter type
    // print_uppercase(...);
    // print_uppercase(...);
}

// TODO: Implement this function to print a string in uppercase
// It should accept &str so it works with both String and &str
// fn print_uppercase(...) {
//     println!("{}", ...);
// }

// Bonus Challenge: Implement a function that counts words
// Uncomment when ready
/*
fn count_words(text: &str) -> usize {
    // TODO: Count the number of words in text
    // Hint: Use .split_whitespace() and .count()
    0
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Uncomment tests as you implement functions

    // #[test]
    // fn test_print_length() {
    //     let s = String::from("test");
    //     print_length(&s); // Should not panic
    //     assert_eq!(s.len(), 4); // s should still be valid
    // }

    // #[test]
    // fn test_add_exclamation() {
    //     let mut s = String::from("Hello");
    //     add_exclamation(&mut s);
    //     assert_eq!(s, "Hello!!!");
    // }

    // #[test]
    // fn test_count_words() {
    //     assert_eq!(count_words("hello world"), 2);
    //     assert_eq!(count_words("one"), 1);
    //     assert_eq!(count_words(""), 0);
    // }
}
