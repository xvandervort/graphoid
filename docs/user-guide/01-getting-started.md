# Chapter 1: Getting Started

Welcome to Graphoid! This guide will get you up and running in just a few minutes.

## What is Graphoid?

Graphoid is a revolutionary programming language where **everything is a graph**. Unlike traditional languages that bolt graphs onto the side, Graphoid makes graphs the fundamental abstraction at every level:

- **Data structures** are graphs: Lists are linked nodes, trees are hierarchical graphs, maps are key-value graphs
- **Variable storage** is a graph: Your namespace is a meta-graph you can inspect and manipulate
- **Runtime environment** is a graph: Functions are nodes in a call graph, modules are connected subgraphs

But don't worry - you don't need to understand graph theory to use Graphoid! The language feels familiar if you've used Python, Ruby, or JavaScript, but with powerful graph operations when you need them.

## Installation

### Prerequisites

- Rust toolchain (for building from source)
- Git (for cloning the repository)

### Building Graphoid

```bash
# Clone the repository
git clone https://github.com/yourusername/graphoid.git
cd graphoid/rust

# Build the release version
cargo build --release

# The binary will be at: target/release/graphoid
```

### Add to PATH (Optional)

```bash
# Linux/macOS
export PATH="$PATH:/path/to/graphoid/rust/target/release"

# Or create a symlink
sudo ln -s /path/to/graphoid/rust/target/release/graphoid /usr/local/bin/graphoid
```

## Your First Graphoid Session

Let's start the interactive REPL (Read-Eval-Print Loop):

```bash
graphoid
```

You should see:

```
Graphoid v0.1.0
Type 'exit' or press Ctrl+D to quit
>>>
```

### Hello, World!

Type this at the prompt:

```graphoid
>>> print("Hello, World!")
Hello, World!
```

Congratulations! You just wrote your first Graphoid program.

### Try Some Math

Graphoid has all the operators you'd expect:

```graphoid
>>> 2 + 2
4

>>> 10 * 5
50

>>> 100 / 4
25

>>> 2 ** 8
256
```

### Variables

Variables in Graphoid are dynamically typed:

```graphoid
>>> name = "Alice"
>>> age = 30
>>> score = 95.5

>>> print("Name: " + name)
Name: Alice

>>> print("Age: " + age.to_string())
Age: 30
```

### Lists

Lists are one of Graphoid's core collection types:

```graphoid
>>> numbers = [1, 2, 3, 4, 5]
>>> print(numbers)
[1, 2, 3, 4, 5]

>>> numbers.append(6)
>>> print(numbers)
[1, 2, 3, 4, 5, 6]

>>> print(numbers.length())
6
```

### Functions

Functions are first-class citizens in Graphoid:

```graphoid
>>> fn greet(name) {
...     return "Hello, " + name + "!"
... }

>>> print(greet("Bob"))
Hello, Bob!
```

**Note**: The REPL automatically detects multi-line input. Just keep typing and press Enter when you're done.

### Exiting the REPL

To exit the REPL:

```graphoid
>>> exit
```

Or press `Ctrl+D` (Linux/macOS) or `Ctrl+Z` then `Enter` (Windows).

## Your First Graphoid Program

Let's write a program in a file. Create a file called `hello.gr`:

```graphoid
# hello.gr - My first Graphoid program

fn greet(name) {
    return "Hello, " + name + "!"
}

# Greet some people
names = ["Alice", "Bob", "Charlie"]

for name in names {
    print(greet(name))
}
```

Run it:

```bash
graphoid hello.gr
```

Output:

```
Hello, Alice!
Hello, Bob!
Hello, Charlie!
```

## File Extension

Graphoid programs use the `.gr` file extension:

- `program.gr` - A Graphoid program
- `module.gr` - A Graphoid module (can be imported)
- `test.spec.gr` - A test file (future feature)

## Comments

Graphoid supports single-line comments with `#`:

```graphoid
# This is a comment
x = 42  # This is also a comment

# Multi-line comments can be done like this:
# Line 1
# Line 2
# Line 3
```

## REPL Tips

### View Variables

Use the REPL to explore what variables are defined:

```graphoid
>>> x = 10
>>> y = 20
>>> x + y
30
```

### Multi-line Editing

The REPL handles multi-line input automatically:

```graphoid
>>> fn calculate(a, b) {
...     sum = a + b
...     product = a * b
...     return [sum, product]
... }

>>> result = calculate(5, 3)
>>> print(result)
[8, 15]
```

### Error Messages

Graphoid provides helpful error messages:

```graphoid
>>> x = 10 / 0
Error: Runtime error: Division by zero

>>> unknown_variable
Error: Runtime error: Undefined variable: unknown_variable
```

## What You've Learned

In this chapter, you've learned how to:

- ✅ Install and run Graphoid
- ✅ Use the interactive REPL
- ✅ Write your first "Hello, World!" program
- ✅ Work with variables, lists, and functions
- ✅ Create and run `.gr` program files
- ✅ Navigate error messages

## Next Steps

In the next chapter, we'll dive deeper into Graphoid's basics:

- **Chapter 2: Basics** - Variables, types, operators, and expressions
- **Chapter 3: Control Flow** - Conditionals, loops, and pattern matching
- **Chapter 4: Functions** - Deep dive into functions, lambdas, and closures

Ready to continue? Let's go!

---

## Quick Reference

### Running Graphoid

```bash
graphoid                 # Start REPL
graphoid program.gr      # Run a program
graphoid --help          # Show help
```

### Essential Commands

```graphoid
print(value)             # Print to console
exit                     # Exit REPL (or Ctrl+D)
```

### Getting Help

- 📖 [Full documentation](https://graphoid.org/docs)
- 💬 [Community forum](https://discuss.graphoid.org)
- 🐛 [Report issues](https://github.com/yourusername/graphoid/issues)

---

**Exercises**

Try these exercises to practice what you've learned:

1. Write a program that prints your name, age, and favorite color
2. Create a list of your top 5 favorite movies and print each one
3. Write a function that takes two numbers and returns their sum and difference
4. Create a program that calculates the area of a circle given its radius

**Solutions** are available in `examples/01-hello-world/exercises.gr`

---

[Next Chapter: Basics →](02-basics.md)
