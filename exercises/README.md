# Rust Exercises

This directory contains hands-on exercises to practice Rust concepts.

## Exercise Overview

1. **01-basics** - Variables, functions, control flow, and loops
2. **02-ownership** - Ownership, borrowing, and references
3. **03-structs-enums** - Structs, enums, methods, and pattern matching
4. **04-error-handling** - Result, Option, and error propagation
5. **05-collections** - Vectors, HashMaps, and iterators

## How to Work Through Exercises

### 1. Setup
First, ensure Rust is installed (see main README.md).

### 2. Navigate to an Exercise
```bash
cd exercises/01-basics
```

### 3. Read the Instructions
Open `src/main.rs` and read the TODOs.

### 4. Run the Exercise
```bash
cargo run
```

### 5. Run Tests
```bash
cargo test
```

### 6. Check Your Code
```bash
# Format your code
cargo fmt

# Run the linter
cargo clippy
```

### 7. Compare with Solution
When you're done (or stuck), check the solution in `solutions/01-basics/`.

## Tips

- **Read compiler errors carefully** - Rust's error messages are very helpful!
- **Start with the tests** - Uncomment tests one at a time and make them pass
- **Use `println!` for debugging** - Add debug statements to understand what's happening
- **Don't skip exercises** - Each builds on previous concepts
- **Experiment** - Try breaking things to understand the rules

## Exercise Difficulty

- 🟢 01-basics: Beginner
- 🟢 02-ownership: Beginner (but new concept!)
- 🟡 03-structs-enums: Intermediate
- 🟡 04-error-handling: Intermediate
- 🟡 05-collections: Intermediate

## Common Commands

```bash
# Create a new exercise (if you want to experiment)
cargo new my_experiment

# Build without running
cargo build

# Build with optimizations
cargo build --release

# Run a specific example
cargo run --example example_name

# Run tests with output
cargo test -- --nocapture

# Update dependencies
cargo update
```

## Getting Help

If you get stuck:

1. Read the error message carefully
2. Check the TUTORIAL.md for concept review
3. Look at the solution in `solutions/`
4. Search the [Rust documentation](https://doc.rust-lang.org/)
5. Ask on [Rust forums](https://users.rust-lang.org/)

## What's Next?

After completing these exercises:

1. Build a CLI tool (try [clap](https://docs.rs/clap/) for argument parsing)
2. Create a simple web API (try [axum](https://docs.rs/axum/) or [actix-web](https://actix.rs/))
3. Solve [Advent of Code](https://adventofcode.com/) challenges in Rust
4. Contribute to open source Rust projects
5. Read [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

Happy coding! 🦀
