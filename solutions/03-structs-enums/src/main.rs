// Exercise 03: Structs and Enums
// SOLUTION

fn main() {
    println!("=== Exercise 1: Define a Struct ===");
    exercise_1();

    println!("\n=== Exercise 2: Methods ===");
    exercise_2();

    println!("\n=== Exercise 3: Enums ===");
    exercise_3();

    println!("\n=== Exercise 4: Option ===");
    exercise_4();

    println!("\n=== Bonus: Result ===");
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn exercise_1() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };

    println!("Name: {}, Age: {}, Email: {}", person.name, person.age, person.email);
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn exercise_2() {
    let rect = Rectangle::new(5.0, 10.0);
    println!("Area: {}", rect.area());
    println!("Is square: {}", rect.is_square());

    let square = Rectangle::new(5.0, 5.0);
    println!("Square area: {}", square.area());
    println!("Is square: {}", square.is_square());
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 45,
        }
    }
}

fn exercise_3() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;

    println!("Red light duration: {} seconds", red.duration());
    println!("Yellow light duration: {} seconds", yellow.duration());
    println!("Green light duration: {} seconds", green.duration());
}

fn find_person<'a>(people: &'a [Person], name: &str) -> Option<&'a Person> {
    for person in people {
        if person.name == name {
            return Some(person);
        }
    }
    None
}

fn exercise_4() {
    let people = vec![
        Person {
            name: String::from("Alice"),
            age: 30,
            email: String::from("alice@example.com"),
        },
        Person {
            name: String::from("Bob"),
            age: 25,
            email: String::from("bob@example.com"),
        },
        Person {
            name: String::from("Charlie"),
            age: 35,
            email: String::from("charlie@example.com"),
        },
    ];

    match find_person(&people, "Alice") {
        Some(person) => println!("Found: {} ({})", person.name, person.email),
        None => println!("Not found"),
    }

    match find_person(&people, "David") {
        Some(person) => println!("Found: {} ({})", person.name, person.email),
        None => println!("Not found"),
    }
}

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(3.0, 4.0);
        assert_eq!(rect.area(), 12.0);
    }

    #[test]
    fn test_rectangle_is_square() {
        let rect = Rectangle::new(5.0, 5.0);
        assert!(rect.is_square());

        let rect2 = Rectangle::new(3.0, 4.0);
        assert!(!rect2.is_square());
    }

    #[test]
    fn test_traffic_light_duration() {
        assert_eq!(TrafficLight::Red.duration(), 30);
        assert_eq!(TrafficLight::Yellow.duration(), 3);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert!(matches!(divide(10.0, 0.0), Err(MathError::DivisionByZero)));
    }
}
