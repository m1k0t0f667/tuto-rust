# Rust Tutorial: Core Concepts

This guide covers Rust fundamentals with comparisons to TypeScript and Python.

## Table of Contents
1. [Variables and Mutability](#1-variables-and-mutability)
2. [Data Types](#2-data-types)
3. [Functions](#3-functions)
4. [Control Flow](#4-control-flow)
5. [Ownership & Borrowing](#5-ownership--borrowing)
6. [Structs and Enums](#6-structs-and-enums)
7. [Error Handling](#7-error-handling)
8. [Collections](#8-collections)
9. [Pattern Matching](#9-pattern-matching)
10. [Traits](#10-traits)

---

## 1. Variables and Mutability

### Rust
Variables are **immutable by default**. Use `mut` for mutability.

```rust
let x = 5;          // immutable
// x = 6;           // ❌ ERROR!

let mut y = 5;      // mutable
y = 6;              // ✅ OK

// Shadowing: redeclare variable with same name
let z = 5;
let z = z + 1;      // ✅ OK, creates new variable
```

### TypeScript Comparison
```typescript
const x = 5;        // Similar to Rust's immutable let
// x = 6;           // ❌ ERROR

let y = 5;          // Similar to Rust's mut
y = 6;              // ✅ OK
```

### Python Comparison
```python
x = 5
x = 6               # Always mutable (reassignable)
```

**Key Insight:** Rust encourages immutability by default for safer code.

---

## 2. Data Types

### Scalar Types

```rust
// Integers: i8, i16, i32, i64, i128, isize (signed)
//           u8, u16, u32, u64, u128, usize (unsigned)
let a: i32 = 42;
let b: u8 = 255;

// Floating point
let x: f64 = 3.14;
let y: f32 = 2.71;

// Boolean
let is_active: bool = true;

// Character (4 bytes, Unicode)
let emoji: char = '🦀';
```

### Compound Types

```rust
// Tuple (fixed size, mixed types)
let tuple: (i32, f64, char) = (42, 3.14, 'x');
let (a, b, c) = tuple;          // destructuring
let first = tuple.0;            // index access

// Array (fixed size, same type)
let arr: [i32; 3] = [1, 2, 3];
let first = arr[0];

// Type inference works!
let x = 42;                     // Rust infers i32
let y = 3.14;                   // Rust infers f64
```

### TypeScript Comparison
```typescript
// Tuple
let tuple: [number, number, string] = [42, 3.14, 'x'];

// Array
let arr: number[] = [1, 2, 3];
```

### Python Comparison
```python
# Tuple
tuple = (42, 3.14, 'x')

# List (dynamic)
arr = [1, 2, 3]
```

---

## 3. Functions

```rust
// Basic function
fn add(x: i32, y: i32) -> i32 {
    x + y                       // No semicolon = return value
}

// With return statement
fn subtract(x: i32, y: i32) -> i32 {
    return x - y;               // Explicit return
}

// No return value (returns unit type `()`)
fn print_hello() {
    println!("Hello!");
}

// Multiple return values via tuple
fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)              // quotient, remainder
}
```

### TypeScript Comparison
```typescript
function add(x: number, y: number): number {
    return x + y;
}

// Arrow function
const add = (x: number, y: number): number => x + y;
```

### Python Comparison
```python
def add(x: int, y: int) -> int:
    return x + y
```

---

## 4. Control Flow

### If Expressions

```rust
let number = 7;

if number < 5 {
    println!("less than 5");
} else if number < 10 {
    println!("between 5 and 10");
} else {
    println!("10 or more");
}

// If is an expression (returns value)
let result = if number > 5 { "big" } else { "small" };
```

### Loops

```rust
// Infinite loop
loop {
    println!("forever!");
    break;                      // Exit loop
}

// While loop
let mut count = 0;
while count < 10 {
    count += 1;
}

// For loop (range)
for i in 0..5 {                 // 0, 1, 2, 3, 4
    println!("{}", i);
}

for i in 0..=5 {                // 0, 1, 2, 3, 4, 5 (inclusive)
    println!("{}", i);
}

// For loop (iterator)
let arr = [1, 2, 3];
for element in arr.iter() {
    println!("{}", element);
}

// Loop with return value
let result = loop {
    count += 1;
    if count == 10 {
        break count * 2;        // Returns 20
    }
};
```

### TypeScript Comparison
```typescript
// For loop
for (let i = 0; i < 5; i++) {
    console.log(i);
}

// For-of loop
for (const element of [1, 2, 3]) {
    console.log(element);
}
```

### Python Comparison
```python
# For loop
for i in range(5):
    print(i)

# For-each
for element in [1, 2, 3]:
    print(element)
```

---

## 5. Ownership & Borrowing

**This is Rust's most unique feature!** No garbage collector, no manual memory management.

### The Ownership Rules

1. Each value has one **owner**
2. When the owner goes out of scope, the value is dropped
3. There can only be one owner at a time

```rust
// String (heap-allocated, growable)
let s1 = String::from("hello");
let s2 = s1;                    // s1 MOVED to s2
// println!("{}", s1);          // ❌ ERROR! s1 no longer valid

// Clone (deep copy)
let s3 = s2.clone();
println!("{} {}", s2, s3);      // ✅ Both valid

// Copy types (stack-only data)
let x = 5;
let y = x;                      // Copied (integers implement Copy)
println!("{} {}", x, y);        // ✅ Both valid
```

### Borrowing (References)

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);    // Borrow (reference)
    println!("{} has length {}", s1, len); // ✅ s1 still valid
}

fn calculate_length(s: &String) -> usize {
    s.len()                             // Read-only access
}
```

### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);                     // Mutable borrow
    println!("{}", s);                  // "hello world"
}

fn change(s: &mut String) {
    s.push_str(" world");
}

// Rules:
// 1. Only ONE mutable reference at a time
// 2. Can't have mutable & immutable references simultaneously
let mut s = String::from("hello");

let r1 = &s;                    // ✅ immutable borrow
let r2 = &s;                    // ✅ another immutable borrow
// let r3 = &mut s;             // ❌ ERROR! can't have mutable while immutable exists

let r1 = &mut s;                // ✅ mutable borrow
// let r2 = &mut s;             // ❌ ERROR! only one mutable borrow allowed
```

### TypeScript/Python Comparison
```typescript
// TypeScript/Python: everything is a reference
let s1 = {value: "hello"};
let s2 = s1;                    // Both point to same object
s2.value = "world";
console.log(s1.value);          // "world" (mutation visible)
```

**Key Insight:** Rust prevents data races and null pointer errors at compile time!

---

## 6. Structs and Enums

### Structs

```rust
// Define struct
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// Create instance
let user = User {
    username: String::from("alice"),
    email: String::from("alice@example.com"),
    age: 30,
    active: true,
};

// Access fields
println!("{}", user.username);

// Mutable instance
let mut user2 = User {
    username: String::from("bob"),
    email: String::from("bob@example.com"),
    age: 25,
    active: true,
};
user2.age = 26;

// Tuple structs
struct Point(i32, i32, i32);
let origin = Point(0, 0, 0);
```

### Methods

```rust
impl User {
    // Method (takes &self)
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // Associated function (like static method)
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            age: 0,
            active: true,
        }
    }
}

// Usage
let user = User::new(String::from("charlie"), String::from("charlie@example.com"));
println!("Is adult? {}", user.is_adult());
```

### Enums

```rust
// Simple enum
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;

// Enum with data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));

// Option<T> (replaces null/undefined/None)
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;

// Must handle None case
if let Some(n) = some_number {
    println!("Number is {}", n);
}
```

### TypeScript Comparison
```typescript
// Interface/Type
interface User {
    username: string;
    email: string;
    age: number;
    active: boolean;
}

// Union type (similar to enum)
type Direction = "North" | "South" | "East" | "West";
```

### Python Comparison
```python
# Class
class User:
    def __init__(self, username: str, email: str):
        self.username = username
        self.email = email
        self.age = 0
        self.active = True

    def is_adult(self) -> bool:
        return self.age >= 18

# Enum (Python 3.4+)
from enum import Enum
class Direction(Enum):
    NORTH = 1
    SOUTH = 2
    EAST = 3
    WEST = 4
```

---

## 7. Error Handling

Rust uses `Result<T, E>` instead of exceptions.

```rust
use std::fs::File;
use std::io::Read;

// Result<T, E> enum
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Handling errors explicitly
fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("hello.txt")?; // ? propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Using match
fn open_file() {
    match File::open("hello.txt") {
        Ok(file) => println!("File opened!"),
        Err(error) => println!("Error: {:?}", error),
    }
}

// Using if let
if let Ok(file) = File::open("hello.txt") {
    println!("File opened!");
}

// unwrap (panics on error - use sparingly!)
let file = File::open("hello.txt").unwrap();

// expect (panic with custom message)
let file = File::open("hello.txt").expect("Failed to open file");
```

### TypeScript Comparison
```typescript
// Try-catch
try {
    const file = await fs.readFile("hello.txt");
} catch (error) {
    console.error(error);
}
```

### Python Comparison
```python
# Try-except
try:
    with open("hello.txt") as file:
        contents = file.read()
except IOError as error:
    print(f"Error: {error}")
```

**Key Insight:** Rust forces you to handle errors at compile time!

---

## 8. Collections

### Vector (Dynamic Array)

```rust
// Create vector
let mut vec: Vec<i32> = Vec::new();
let vec = vec![1, 2, 3];        // Macro

// Add elements
vec.push(4);
vec.push(5);

// Access elements
let third = vec[2];             // Panics if out of bounds
let third = vec.get(2);         // Returns Option<&i32>

match vec.get(2) {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}

// Iterate
for i in &vec {
    println!("{}", i);
}

// Mutable iteration
for i in &mut vec {
    *i += 10;
}
```

### HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Insert
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Get value
let team = String::from("Blue");
let score = scores.get(&team);  // Returns Option<&i32>

match score {
    Some(s) => println!("Score: {}", s),
    None => println!("Team not found"),
}

// Iterate
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Update
scores.insert(String::from("Blue"), 25); // Overwrites

// Insert if not exists
scores.entry(String::from("Yellow")).or_insert(50);
```

### TypeScript Comparison
```typescript
// Array
let arr: number[] = [1, 2, 3];
arr.push(4);

// Map
let map = new Map<string, number>();
map.set("Blue", 10);
map.get("Blue"); // Returns number | undefined
```

### Python Comparison
```python
# List
arr = [1, 2, 3]
arr.append(4)

# Dictionary
scores = {"Blue": 10, "Red": 50}
scores["Blue"]  # Returns value or raises KeyError
scores.get("Blue")  # Returns value or None
```

---

## 9. Pattern Matching

### Match Expression

```rust
// Basic match
let number = 3;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    3 => println!("Three"),
    _ => println!("Something else"), // _ is catch-all
}

// Match with return value
let result = match number {
    1 => "one",
    2 => "two",
    3 => "three",
    _ => "other",
};

// Match Option
let maybe_number: Option<i32> = Some(5);

match maybe_number {
    Some(n) => println!("Number is {}", n),
    None => println!("No number"),
}

// Match with conditions (guards)
let pair = (2, -2);

match pair {
    (x, y) if x == y => println!("Equal"),
    (x, y) if x + y == 0 => println!("Sum is zero"),
    (x, _) if x % 2 == 0 => println!("First is even"),
    _ => println!("Something else"),
}

// Destructuring
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 0, y: 7 };

match point {
    Point { x: 0, y } => println!("On Y axis at {}", y),
    Point { x, y: 0 } => println!("On X axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}
```

### If Let

```rust
// Shorter syntax for matching one pattern
let some_value: Option<i32> = Some(3);

if let Some(n) = some_value {
    println!("Number is {}", n);
} else {
    println!("No number");
}

// Equivalent to:
match some_value {
    Some(n) => println!("Number is {}", n),
    _ => println!("No number"),
}
```

### TypeScript Comparison
```typescript
// Switch statement (less powerful)
switch (number) {
    case 1:
        console.log("One");
        break;
    case 2:
        console.log("Two");
        break;
    default:
        console.log("Other");
}

// Destructuring
const {x, y} = point;
```

### Python Comparison
```python
# Pattern matching (Python 3.10+)
match number:
    case 1:
        print("One")
    case 2:
        print("Two")
    case _:
        print("Other")
```

---

## 10. Traits

Traits define shared behavior (similar to interfaces).

```rust
// Define trait
trait Summary {
    fn summarize(&self) -> String;

    // Default implementation
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement trait for struct
struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, self.content)
    }
}

// Using traits as parameters
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bounds (generic)
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Common traits
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

let p1 = Point { x: 1, y: 2 };
let p2 = p1.clone();
println!("{:?}", p1);           // Debug print
println!("{}", p1 == p2);       // Equality
```

### TypeScript Comparison
```typescript
// Interface
interface Summary {
    summarize(): string;
}

class Article implements Summary {
    constructor(public title: string, public content: string) {}

    summarize(): string {
        return `${this.title}: ${this.content}`;
    }
}

function notify(item: Summary) {
    console.log(item.summarize());
}
```

### Python Comparison
```python
# Protocol (Python 3.8+) or ABC
from typing import Protocol

class Summary(Protocol):
    def summarize(self) -> str:
        ...

class Article:
    def __init__(self, title: str, content: str):
        self.title = title
        self.content = content

    def summarize(self) -> str:
        return f"{self.title}: {self.content}"
```

---

## Next Steps

1. Work through the exercises in `exercises/`
2. Read [The Rust Book](https://doc.rust-lang.org/book/) - especially:
   - Chapter 4: Ownership
   - Chapter 10: Generics, Traits, Lifetimes
   - Chapter 13: Iterators and Closures
3. Try [Rustlings](https://github.com/rust-lang/rustlings) for more practice
4. Build a small CLI tool or web server

Remember: The Rust compiler is your friend. Read the error messages carefully!
