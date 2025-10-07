// Exercise 05: Collections
//
// Master Rust's powerful collection types: Vec, HashMap, and more

use std::collections::HashMap;

fn main() {
    println!("=== Exercise 1: Vectors ===");
    exercise_1();

    println!("\n=== Exercise 2: HashMaps ===");
    exercise_2();

    println!("\n=== Exercise 3: Iterators ===");
    exercise_3();

    println!("\n=== Exercise 4: Practical Application ===");
    exercise_4();
}

// Exercise 1: Working with Vectors
fn exercise_1() {
    // TODO: Create a vector of integers from 1 to 5
    // let mut numbers = vec![...];

    // TODO: Add 6 and 7 to the vector

    // TODO: Remove the last element

    // TODO: Print the length of the vector

    // TODO: Iterate and print each number multiplied by 2
}

// Exercise 2: Working with HashMaps
fn exercise_2() {
    // TODO: Create a HashMap to store product prices
    // Key: product name (String), Value: price (f64)
    // let mut prices = HashMap::new();

    // TODO: Insert at least 3 products with prices

    // TODO: Look up a product and print its price

    // TODO: Update a product's price

    // TODO: Iterate over all products and print them
}

// Exercise 3: Using Iterators
fn exercise_3() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // TODO: Use .iter() and .filter() to get even numbers only
    // let evens: Vec<&i32> = numbers.iter()...;

    // TODO: Use .iter() and .map() to square each number
    // let squared: Vec<i32> = numbers.iter()...;

    // TODO: Use .iter() and .sum() to calculate the sum
    // let sum: i32 = numbers.iter()...;

    // TODO: Use .iter() and .fold() to calculate the product
    // let product: i32 = numbers.iter()...;
}

// Exercise 4: Word Frequency Counter
// TODO: Implement a function that counts word frequency in a text
// fn count_word_frequency(text: &str) -> HashMap<String, usize> {
//     // Hint: Use .split_whitespace(), .to_lowercase(), and HashMap
//     // For each word, either insert 1 or increment existing count
// }

fn exercise_4() {
    let text = "the quick brown fox jumps over the lazy dog the fox was quick";

    // TODO: Call count_word_frequency and print results
    // let frequencies = count_word_frequency(text);
    // for (word, count) in &frequencies {
    //     println!("{}: {}", word, count);
    // }

    // TODO: Find the most common word
    // Hint: Use .iter().max_by_key()
}

// Bonus Challenge: Implement a simple phonebook
// Uncomment when ready
/*
struct PhoneBook {
    contacts: HashMap<String, String>,
}

impl PhoneBook {
    fn new() -> Self {
        // TODO: Create new empty phonebook
    }

    fn add_contact(&mut self, name: String, number: String) {
        // TODO: Add a contact
    }

    fn get_number(&self, name: &str) -> Option<&String> {
        // TODO: Look up a phone number
    }

    fn remove_contact(&mut self, name: &str) -> bool {
        // TODO: Remove a contact, return true if existed
    }

    fn list_contacts(&self) {
        // TODO: Print all contacts
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Uncomment tests as you implement functions

    // #[test]
    // fn test_count_word_frequency() {
    //     let text = "hello world hello";
    //     let freq = count_word_frequency(text);
    //     assert_eq!(freq.get("hello"), Some(&2));
    //     assert_eq!(freq.get("world"), Some(&1));
    // }

    // #[test]
    // fn test_phonebook() {
    //     let mut pb = PhoneBook::new();
    //     pb.add_contact("Alice".to_string(), "123-456".to_string());
    //     assert_eq!(pb.get_number("Alice"), Some(&"123-456".to_string()));
    //     assert_eq!(pb.get_number("Bob"), None);
    //     assert!(pb.remove_contact("Alice"));
    //     assert!(!pb.remove_contact("Bob"));
    // }
}
