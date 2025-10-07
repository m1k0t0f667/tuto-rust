# Rust Tutorial for TypeScript/Python Developers

Welcome to Rust! This tutorial is designed for developers with experience in TypeScript and Python who want to learn Rust.

## Table of Contents
- [Environment Setup](#environment-setup)
- [Getting Started](#getting-started)
- [Tutorial Structure](#tutorial-structure)
- [Resources](#resources)

## Environment Setup

### 1. Install Rust

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run [rustup-init.exe](https://rustup.rs/)

After installation, restart your terminal and verify:
```bash
rustc --version
cargo --version
```

### 2. Configure Your PATH

The installer should automatically add Rust to your PATH. If not, add this to your shell profile:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### 3. Install Essential Tools

```bash
# Rust formatter
rustup component add rustfmt

# Rust linter
rustup component add clippy

# Rust Language Server (for IDE support)
rustup component add rust-analyzer
```

### 4. IDE Setup

**VS Code (Recommended):**
- Install the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) extension
- Install [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging

**Other IDEs:**
- IntelliJ IDEA: Install the Rust plugin
- Vim/Neovim: Use rust-analyzer with your LSP client

### 5. Verify Installation

```bash
cargo new hello-rust
cd hello-rust
cargo run
```

You should see "Hello, world!" printed to the console.

## Getting Started

### Understanding Cargo

Cargo is Rust's package manager and build system (similar to npm/pip):

```bash
cargo new project_name      # Create new project
cargo build                 # Compile project
cargo run                   # Compile and run
cargo test                  # Run tests
cargo check                 # Check code without building
cargo clippy                # Run linter
cargo fmt                   # Format code
```

### Project Structure

```
my_project/
├── Cargo.toml           # Package manifest (like package.json/requirements.txt)
├── Cargo.lock           # Lock file (like package-lock.json/poetry.lock)
└── src/
    └── main.rs          # Entry point
```

## Tutorial Structure

This tutorial is organized into progressive lessons:

1. **TUTORIAL.md** - Core concepts and syntax guide
2. **exercises/** - Hands-on coding exercises
   - Each exercise has a description and starter code
   - Solutions are provided in `solutions/`

### Working Through Exercises

```bash
# Navigate to an exercise
cd exercises/01-basics

# Run the exercise
cargo run

# Run tests
cargo test

# Check your solution
cargo clippy
```

## Key Differences from TypeScript/Python

| Concept | TypeScript/Python | Rust |
|---------|------------------|------|
| **Type System** | Optional/Duck typing | Strong static typing |
| **Memory** | Garbage collected | Ownership & borrowing |
| **Null** | `null`/`undefined`/`None` | `Option<T>` enum |
| **Error Handling** | Exceptions/try-catch | `Result<T, E>` enum |
| **Mutability** | Mutable by default | Immutable by default |
| **Compilation** | Runtime (or transpiled) | Compiled to native code |

## Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/) - The definitive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [Standard Library Docs](https://doc.rust-lang.org/std/)

### Interactive Learning
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust)

### Coming from Other Languages
- [Rust for TypeScript Developers](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)
- [Rust vs Python](https://www.youtube.com/watch?v=F3A7-wP0c_k)

### Community
- [Official Rust Forum](https://users.rust-lang.org/)
- [/r/rust](https://reddit.com/r/rust)
- [Rust Discord](https://discord.gg/rust-lang)

## Tips for Success

1. **Embrace the compiler** - Rust's compiler errors are incredibly helpful. Read them carefully!
2. **Start small** - Don't try to fight the borrow checker on complex projects initially
3. **Use clippy** - `cargo clippy` gives excellent suggestions
4. **Read error messages** - They often include suggestions for fixes
5. **Don't skip the Rust Book** - Chapters 4 (Ownership) and 10 (Generics) are crucial

## Next Steps

1. Complete the environment setup above
2. Read through `TUTORIAL.md` for core concepts
3. Work through exercises in order (01-basics → 02-ownership → etc.)
4. Build a small project to consolidate your learning

Happy coding! 🦀
