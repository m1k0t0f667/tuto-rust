// Exercise 04: Error Handling
// SOLUTION

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

    println!("\n=== Bonus: Chain Operations ===");
    match process_number("42") {
        Ok(n) => println!("Result: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn exercise_1() {
    let numbers = vec!["42", "abc", "100", "xyz"];

    for num_str in numbers {
        match num_str.parse::<i32>() {
            Ok(num) => println!("Parsed: {}", num),
            Err(_) => println!("Failed to parse: {}", num_str),
        }
    }
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn exercise_2() {
    // Create a test file
    let _ = fs::write("test.txt", "Hello from Rust!\nThis is a test file.");

    match read_file_contents("test.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    match read_file_contents("nonexistent.txt") {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Clean up
    let _ = fs::remove_file("test.txt");
}

#[derive(Debug, PartialEq)]
enum ValidationError {
    TooShort,
    TooLong,
    InvalidCharacter,
}

fn validate_username(username: &str) -> Result<(), ValidationError> {
    if username.len() < 3 {
        return Err(ValidationError::TooShort);
    }
    if username.len() > 20 {
        return Err(ValidationError::TooLong);
    }
    if !username.chars().all(|c| c.is_alphanumeric()) {
        return Err(ValidationError::InvalidCharacter);
    }
    Ok(())
}

fn exercise_3() {
    let usernames = vec!["ab", "alice123", "bob_invalid", "verylongusernamethatexceedslimit"];

    for username in usernames {
        match validate_username(username) {
            Ok(_) => println!("'{}' is valid", username),
            Err(e) => println!("'{}' is invalid: {:?}", username, e),
        }
    }
}

fn get_age_string(name: &str) -> Option<&str> {
    match name {
        "Alice" => Some("30"),
        "Bob" => Some("25"),
        "Charlie" => Some("invalid"),
        _ => None,
    }
}

fn get_user_age(name: &str) -> Result<u32, String> {
    let age_str = get_age_string(name)
        .ok_or_else(|| format!("User '{}' not found", name))?;

    age_str
        .parse::<u32>()
        .map_err(|_| format!("Invalid age format for user '{}'", name))
}

fn exercise_4() {
    let names = vec!["Alice", "Bob", "Charlie", "David"];

    for name in names {
        match get_user_age(name) {
            Ok(age) => println!("{} is {} years old", name, age),
            Err(e) => println!("Error for {}: {}", name, e),
        }
    }
}

fn process_number(input: &str) -> Result<i32, String> {
    let num = input
        .parse::<i32>()
        .map_err(|_| format!("Failed to parse '{}' as number", input))?;

    let doubled = num * 2;

    if doubled <= 0 {
        return Err(format!("Result {} is not positive", doubled));
    }

    Ok(doubled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_username() {
        assert!(validate_username("alice").is_ok());
        assert!(matches!(
            validate_username("ab"),
            Err(ValidationError::TooShort)
        ));
        assert!(matches!(
            validate_username("verylongusernamethatexceedslimit"),
            Err(ValidationError::TooLong)
        ));
        assert!(matches!(
            validate_username("bob_invalid"),
            Err(ValidationError::InvalidCharacter)
        ));
    }

    #[test]
    fn test_get_user_age() {
        assert_eq!(get_user_age("Alice"), Ok(30));
        assert_eq!(get_user_age("Bob"), Ok(25));
        assert!(get_user_age("Charlie").is_err());
        assert!(get_user_age("David").is_err());
    }

    #[test]
    fn test_process_number() {
        assert_eq!(process_number("21"), Ok(42));
        assert!(process_number("abc").is_err());
        assert!(process_number("-10").is_err()); // -10 * 2 = -20, not positive
    }
}
