// Exercise 01: Basics - Variables, Functions, and Control Flow
//
// Complete the TODOs below to practice Rust basics

fn main() {
    println!("=== Exercise 1: Variables ===");
    exercise_1();

    println!("\n=== Exercise 2: Functions ===");
    exercise_2();

    println!("\n=== Exercise 3: Control Flow ===");
    exercise_3();

    println!("\n=== Exercise 4: Loops ===");
    exercise_4();
}

// Exercise 1: Create variables with correct mutability
fn exercise_1() {
    // TODO: Create an immutable variable 'name' with your name
    // let name = ...;

    // TODO: Create a mutable variable 'age' with value 25
    // let mut age = ...;

    // TODO: Increment age by 1
    // age = ...;

    // TODO: Uncomment these lines when done
    // println!("Name: {}, Age: {}", name, age);
}

// Exercise 2: Complete the function
fn exercise_2() {
    // TODO: Call the multiply function and print the result
    // let result = multiply(..., ...);
    // println!("Result: {}", result);
}

// TODO: Implement this function to multiply two numbers
fn multiply(a: i32, b: i32) -> i32 {
    // Your code here
    0 // Replace this
}

// Exercise 3: Write if/else logic
fn exercise_3() {
    let temperature = 25;

    // TODO: Write an if/else statement that prints:
    // - "Hot" if temperature > 30
    // - "Warm" if temperature is between 20 and 30 (inclusive)
    // - "Cold" if temperature < 20

    // Your code here
}

// Exercise 4: Practice with loops
fn exercise_4() {
    // TODO: Use a for loop to print numbers 1 to 5
    // for i in ... {
    //     println!("{}", i);
    // }

    // TODO: Use a for loop to iterate over this array and print each element
    let numbers = [10, 20, 30, 40, 50];
    // Your code here

    // TODO: Calculate the sum of all numbers in the array
    let mut sum = 0;
    // Your code here
    // println!("Sum: {}", sum);
}

// Bonus Challenge: FizzBuzz
// Uncomment and implement when ready
/*
fn fizzbuzz(n: i32) {
    // TODO: Print numbers from 1 to n, but:
    // - Print "Fizz" for multiples of 3
    // - Print "Buzz" for multiples of 5
    // - Print "FizzBuzz" for multiples of both 3 and 5
    // - Print the number otherwise
}
*/

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
