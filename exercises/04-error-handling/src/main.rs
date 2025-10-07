// Exercise 04: Error Handling
//
// Learn to handle errors the Rust way with Result and Option

use std::fs;
use std::io;

fn main() {
    println!("=== Exercise 1: Basic Result ===");
    exercise_1();

    println!("\n=== Exercise 2: The ? Operator ===");
    exercise_2();

    println!("\n=== Exercise 3: Custom Errors ===");
    exercise_3();

    println!("\n=== Exercise 4: Combining Option and Result ===");
    exercise_4();
}

// Exercise 1: Parse a string to integer and handle errors
fn exercise_1() {
    let numbers = vec!["42", "abc", "100", "xyz"];

    for num_str in numbers {
        // TODO: Parse num_str to i32 and handle the Result
        // Use match to print either "Parsed: X" or "Failed to parse: Y"

        // Hint: num_str.parse::<i32>() returns Result<i32, ParseIntError>
    }
}

// Exercise 2: Read a file with the ? operator
// TODO: Implement this function to read a file and return its contents
// Use the ? operator to propagate errors
// fn read_file_contents(path: &str) -> Result<String, io::Error> {
//     // Hint: Use fs::read_to_string(path)?
// }

fn exercise_2() {
    // TODO: Call read_file_contents and handle the result
    // Try with a file that exists and one that doesn't

    // Example test file (you can create this):
    // match read_file_contents("test.txt") {
    //     Ok(contents) => println!("File contents:\n{}", contents),
    //     Err(e) => println!("Error reading file: {}", e),
    // }
}

// Exercise 3: Create a custom error type
// TODO: Define an enum for custom errors
// #[derive(Debug)]
// enum ValidationError {
//     TooShort,
//     TooLong,
//     InvalidCharacter,
// }

// TODO: Implement a function that validates a username
// Rules:
// - Must be 3-20 characters long
// - Must only contain alphanumeric characters
// fn validate_username(username: &str) -> Result<(), ValidationError> {
//     ...
// }

fn exercise_3() {
    let usernames = vec!["ab", "alice123", "bob_invalid", "verylongusernamethatexceedslimit"];

    // TODO: Test validate_username with each username
    // for username in usernames {
    //     match validate_username(username) {
    //         Ok(_) => println!("'{}' is valid", username),
    //         Err(e) => println!("'{}' is invalid: {:?}", username, e),
    //     }
    // }
}

// Exercise 4: Work with nested Options and Results
// TODO: Implement a function that looks up a user's age from a database
// The database might not contain the user (Option)
// Parsing the age might fail (Result)

// Mock database
// fn get_age_string(name: &str) -> Option<&str> {
//     match name {
//         "Alice" => Some("30"),
//         "Bob" => Some("25"),
//         "Charlie" => Some("invalid"),
//         _ => None,
//     }
// }

// TODO: Implement a function that gets a user's age as a number
// fn get_user_age(name: &str) -> Result<u32, String> {
//     // Hint: Use get_age_string, then parse the result
//     // Return Err with appropriate message if user not found or parsing fails
// }

fn exercise_4() {
    let names = vec!["Alice", "Bob", "Charlie", "David"];

    // TODO: Test get_user_age with each name
    // for name in names {
    //     match get_user_age(name) {
    //         Ok(age) => println!("{} is {} years old", name, age),
    //         Err(e) => println!("Error for {}: {}", name, e),
    //     }
    // }
}

// Bonus Challenge: Chain multiple fallible operations
// Uncomment when ready
/*
fn process_number(input: &str) -> Result<i32, String> {
    // TODO: Parse input to i32, multiply by 2, then check if result is positive
    // Return appropriate error messages for each failure case
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Uncomment tests as you implement functions

    // #[test]
    // fn test_validate_username() {
    //     assert!(validate_username("alice").is_ok());
    //     assert!(matches!(validate_username("ab"), Err(ValidationError::TooShort)));
    //     assert!(matches!(validate_username("verylongusernamethatexceedslimit"), Err(ValidationError::TooLong)));
    // }

    // #[test]
    // fn test_get_user_age() {
    //     assert_eq!(get_user_age("Alice"), Ok(30));
    //     assert_eq!(get_user_age("Bob"), Ok(25));
    //     assert!(get_user_age("Charlie").is_err());
    //     assert!(get_user_age("David").is_err());
    // }
}
