# Lesson 2: Ownership and Borrowing

## Objectives
- Understand Rust's ownership model
- Learn rules of borrowing
- Apply ownership concepts to functions

## Key Concepts Covered
- Ownership rules and move semantics
- Stack vs heap allocation
- References and borrowing
- Mutable vs immutable references
- Multiple references rules

## Examples in the Code
1. **Basic Ownership Example**
   - Demonstrates ownership transfer
   - Shows how moved values can't be used

2. **Copy Types Example**
   - Shows how primitive types implement Copy
   - Demonstrates stack-only data handling

3. **Cloning Example**
   - Shows deep copying with clone()
   - Demonstrates when both variables can own similar data

4. **Ownership with Functions**
   - Shows how functions take ownership
   - Demonstrates Copy types in function calls

5. **References and Borrowing**
   - Shows immutable and mutable borrowing
   - Demonstrates reference-based function parameters

6. **Multiple References Example**
   - Shows rules about multiple references
   - Demonstrates scope-based reference management

## How to Run
1. Navigate to this directory
2. Run `cargo build` to compile
3. Run `cargo run` to execute the program

## Learning Exercises
1. Try uncommenting the commented-out println! statements to see ownership errors
2. Experiment with creating multiple mutable references (this should fail)
3. Try creating a function that takes ownership and returns ownership
4. Practice creating functions that borrow references instead of taking ownership
