// Exercise 01: Basics - Variables, Functions, and Control Flow
// SOLUTION

fn main() {
    println!("=== Exercise 1: Variables ===");
    exercise_1();

    println!("\n=== Exercise 2: Functions ===");
    exercise_2();

    println!("\n=== Exercise 3: Control Flow ===");
    exercise_3();

    println!("\n=== Exercise 4: Loops ===");
    exercise_4();

    println!("\n=== Bonus: FizzBuzz ===");
    fizzbuzz(15);
}

fn exercise_1() {
    let name = "Alice";
    let mut age = 25;
    age = age + 1;
    println!("Name: {}, Age: {}", name, age);
}

fn exercise_2() {
    let result = multiply(5, 7);
    println!("Result: {}", result);
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn exercise_3() {
    let temperature = 25;

    if temperature > 30 {
        println!("Hot");
    } else if temperature >= 20 {
        println!("Warm");
    } else {
        println!("Cold");
    }
}

fn exercise_4() {
    for i in 1..=5 {
        println!("{}", i);
    }

    let numbers = [10, 20, 30, 40, 50];
    for num in numbers.iter() {
        println!("{}", num);
    }

    let mut sum = 0;
    for num in numbers.iter() {
        sum += num;
    }
    println!("Sum: {}", sum);
}

fn fizzbuzz(n: i32) {
    for i in 1..=n {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 100), 0);
    }
}
