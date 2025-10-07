// Exercise 03: Structs and Enums
//
// Learn to define and use custom types in Rust

fn main() {
    println!("=== Exercise 1: Define a Struct ===");
    exercise_1();

    println!("\n=== Exercise 2: Methods ===");
    exercise_2();

    println!("\n=== Exercise 3: Enums ===");
    exercise_3();

    println!("\n=== Exercise 4: Option ===");
    exercise_4();
}

// Exercise 1: Define a Person struct
// TODO: Define a struct called Person with fields:
// - name: String
// - age: u32
// - email: String

fn exercise_1() {
    // TODO: Create an instance of Person
    // let person = Person {
    //     ...
    // };

    // TODO: Print the person's details
    // println!("Name: {}, Age: {}, Email: {}", ...);
}

// Exercise 2: Add methods to Rectangle
// TODO: Define a Rectangle struct with width and height (f64)
// Then implement methods:
// - area() -> f64 (returns width * height)
// - is_square() -> bool (returns true if width == height)
// - new(width: f64, height: f64) -> Rectangle (constructor)

fn exercise_2() {
    // TODO: Create a Rectangle and test the methods
    // let rect = Rectangle::new(5.0, 10.0);
    // println!("Area: {}", rect.area());
    // println!("Is square: {}", rect.is_square());
}

// Exercise 3: Work with Enums
// TODO: Define an enum called TrafficLight with variants:
// - Red
// - Yellow
// - Green

// TODO: Implement a method that returns the duration (in seconds) for each light
// impl TrafficLight {
//     fn duration(&self) -> u32 {
//         match self {
//             ...
//         }
//     }
// }

fn exercise_3() {
    // TODO: Create traffic light instances and print their durations
    // let red = TrafficLight::Red;
    // println!("Red light duration: {} seconds", red.duration());
}

// Exercise 4: Working with Option
// TODO: Implement a function that finds a person by name in a vector
// Return Option<&Person> (Some if found, None if not found)

fn exercise_4() {
    // TODO: Create a vector of Person structs
    // let people = vec![...];

    // TODO: Test the find_person function
    // match find_person(&people, "Alice") {
    //     Some(person) => println!("Found: {}", person.name),
    //     None => println!("Not found"),
    // }
}

// TODO: Implement find_person function
// fn find_person<'a>(people: &'a [Person], name: &str) -> Option<&'a Person> {
//     ...
// }

// Bonus Challenge: Create a Result-based division function
// Uncomment when ready
/*
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    // TODO: Return Ok(a/b) if b != 0, otherwise Err(DivisionByZero)
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    // Uncomment tests as you implement code

    // #[test]
    // fn test_rectangle_area() {
    //     let rect = Rectangle::new(3.0, 4.0);
    //     assert_eq!(rect.area(), 12.0);
    // }

    // #[test]
    // fn test_rectangle_is_square() {
    //     let rect = Rectangle::new(5.0, 5.0);
    //     assert!(rect.is_square());
    //
    //     let rect2 = Rectangle::new(3.0, 4.0);
    //     assert!(!rect2.is_square());
    // }

    // #[test]
    // fn test_traffic_light_duration() {
    //     assert_eq!(TrafficLight::Red.duration(), 30);
    //     assert_eq!(TrafficLight::Yellow.duration(), 3);
    //     assert_eq!(TrafficLight::Green.duration(), 45);
    // }

    // #[test]
    // fn test_divide() {
    //     assert_eq!(divide(10.0, 2.0), Ok(5.0));
    //     assert!(matches!(divide(10.0, 0.0), Err(MathError::DivisionByZero)));
    // }
}
