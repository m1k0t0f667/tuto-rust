// Exercise 05: Collections
// SOLUTION

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

    println!("\n=== Bonus: PhoneBook ===");
    bonus();
}

fn exercise_1() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.push(6);
    numbers.push(7);

    numbers.pop();

    println!("Length: {}", numbers.len());

    for num in &numbers {
        println!("{} * 2 = {}", num, num * 2);
    }
}

fn exercise_2() {
    let mut prices = HashMap::new();

    prices.insert(String::from("Apple"), 1.50);
    prices.insert(String::from("Banana"), 0.80);
    prices.insert(String::from("Orange"), 1.20);

    if let Some(price) = prices.get("Apple") {
        println!("Apple costs: ${:.2}", price);
    }

    prices.insert(String::from("Apple"), 1.75);

    for (product, price) in &prices {
        println!("{}: ${:.2}", product, price);
    }
}

fn exercise_3() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", evens);

    let squared: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared: {:?}", squared);

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let product: i32 = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);
}

fn count_word_frequency(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *frequencies.entry(word).or_insert(0) += 1;
    }

    frequencies
}

fn exercise_4() {
    let text = "the quick brown fox jumps over the lazy dog the fox was quick";

    let frequencies = count_word_frequency(text);
    println!("Word frequencies:");
    for (word, count) in &frequencies {
        println!("{}: {}", word, count);
    }

    if let Some((word, count)) = frequencies.iter().max_by_key(|(_, &count)| count) {
        println!("\nMost common word: '{}' (appears {} times)", word, count);
    }
}

struct PhoneBook {
    contacts: HashMap<String, String>,
}

impl PhoneBook {
    fn new() -> Self {
        PhoneBook {
            contacts: HashMap::new(),
        }
    }

    fn add_contact(&mut self, name: String, number: String) {
        self.contacts.insert(name, number);
    }

    fn get_number(&self, name: &str) -> Option<&String> {
        self.contacts.get(name)
    }

    fn remove_contact(&mut self, name: &str) -> bool {
        self.contacts.remove(name).is_some()
    }

    fn list_contacts(&self) {
        if self.contacts.is_empty() {
            println!("No contacts");
            return;
        }

        println!("Contacts:");
        for (name, number) in &self.contacts {
            println!("{}: {}", name, number);
        }
    }
}

fn bonus() {
    let mut pb = PhoneBook::new();
    pb.add_contact("Alice".to_string(), "123-456-7890".to_string());
    pb.add_contact("Bob".to_string(), "987-654-3210".to_string());
    pb.add_contact("Charlie".to_string(), "555-555-5555".to_string());

    pb.list_contacts();

    if let Some(number) = pb.get_number("Alice") {
        println!("\nAlice's number: {}", number);
    }

    println!("\nRemoving Bob...");
    pb.remove_contact("Bob");
    pb.list_contacts();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_word_frequency() {
        let text = "hello world hello";
        let freq = count_word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
    }

    #[test]
    fn test_phonebook() {
        let mut pb = PhoneBook::new();
        pb.add_contact("Alice".to_string(), "123-456".to_string());
        assert_eq!(pb.get_number("Alice"), Some(&"123-456".to_string()));
        assert_eq!(pb.get_number("Bob"), None);
        assert!(pb.remove_contact("Alice"));
        assert!(!pb.remove_contact("Bob"));
    }

    #[test]
    fn test_phonebook_update() {
        let mut pb = PhoneBook::new();
        pb.add_contact("Alice".to_string(), "111-111".to_string());
        pb.add_contact("Alice".to_string(), "222-222".to_string());
        assert_eq!(pb.get_number("Alice"), Some(&"222-222".to_string()));
    }
}
