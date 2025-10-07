// Exercise 02: Ownership and Borrowing
// SOLUTION

fn main() {
    println!("=== Exercise 1: Understanding Ownership ===");
    exercise_1();

    println!("\n=== Exercise 2: Borrowing ===");
    exercise_2();

    println!("\n=== Exercise 3: Mutable References ===");
    exercise_3();

    println!("\n=== Exercise 4: String vs &str ===");
    exercise_4();

    println!("\n=== Bonus: Count Words ===");
    let text = "The quick brown fox jumps";
    println!("Word count: {}", count_words(text));
}

fn exercise_1() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Clone instead of move

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn exercise_2() {
    let text = String::from("Rust is awesome");
    print_length(&text); // Borrow with &
    println!("Text is still available: {}", text);
}

fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn exercise_3() {
    let mut message = String::from("Hello");
    add_exclamation(&mut message);
    println!("Modified message: {}", message);
}

fn add_exclamation(s: &mut String) {
    s.push_str("!!!");
}

fn exercise_4() {
    let string = String::from("owned string");
    let str_slice = "string slice";

    print_uppercase(&string);
    print_uppercase(str_slice);
}

fn print_uppercase(s: &str) {
    println!("{}", s.to_uppercase());
}

fn count_words(text: &str) -> usize {
    if text.trim().is_empty() {
        return 0;
    }
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_length() {
        let s = String::from("test");
        print_length(&s);
        assert_eq!(s.len(), 4);
    }

    #[test]
    fn test_add_exclamation() {
        let mut s = String::from("Hello");
        add_exclamation(&mut s);
        assert_eq!(s, "Hello!!!");
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one"), 1);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("  spaces  "), 1);
    }
}
