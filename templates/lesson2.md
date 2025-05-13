## Lesson 2: Ownership and Borrowing

### Objectives:
- Understand Rust's ownership model
- Learn rules of borrowing
- Apply ownership concepts to functions

### Key Concepts:
- Ownership rules
- Move semantics
- References & borrowing
- Mutable vs. immutable references

### Detailed Learning Plan:

#### 1. Understanding Ownership
- Memory management without garbage collection
- Stack vs heap allocation
- The three ownership rules:
  1. Each value in Rust has a variable that's its owner
  2. There can only be one owner at a time
  3. When the owner goes out of scope, the value is dropped

#### 2. Move Semantics vs Copying
- Moving values between variables
- Types that implement the Copy trait
- Cloning for deep copies

#### 3. Borrowing and References
- Creating references with `&`
- Mutable references with `&mut`
- Reference rules:
  1. Any number of immutable references OR
  2. Exactly one mutable reference (not both)
- Preventing data races at compile time

**Sample Code:**
```rust
fn main() {
    // Ownership and moving
    let s1 = String::from("hello");
    let s2 = s1; 
    // println!("{}", s1); // Error: s1's value moved to s2
    
    // Cloning instead of moving
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy
    println!("s3: {}, s4: {}", s3, s4); // Works fine
    
    // Borrowing with immutable references
    let s5 = String::from("reference");
    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);
    
    // Mutable borrowing
    let mut s6 = String::from("hello");
    change(&mut s6);
    println!("Changed string: {}", s6);
    
    // Cannot have multiple mutable references
    let mut s7 = String::from("multiple");
    let r1 = &mut s7;
    // let r2 = &mut s7; // Error: cannot borrow `s7` as mutable more than once
    println!("{}", r1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s is borrowed, not owned, so it isn't dropped here
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Practice Problems:

1. **First Word Finder:**
   Write a function that takes a string slice and returns the first word (defined as characters up to the first space).

   ```rust
   fn first_word(s: &str) -> &str {
       let bytes = s.as_bytes();
       
       for (i, &item) in bytes.iter().enumerate() {
           if item == b' ' {
               return &s[0..i];
           }
       }
       
       &s[..] // Return the whole string if no space found
   }
   
   fn main() {
       let my_string = String::from("Hello world");
       let first = first_word(&my_string);
       println!("First word: {}", first);
   }
   ```

2. **Vector Modifier:**
   Implement a function that modifies a vector in place without taking ownership.

   ```rust
   fn double_values(numbers: &mut Vec<i32>) {
       for i in 0..numbers.len() {
           numbers[i] *= 2;
       }
   }
   
   fn main() {
       let mut my_numbers = vec![1, 2, 3, 4, 5];
       println!("Before: {:?}", my_numbers);
       double_values(&mut my_numbers);
       println!("After: {:?}", my_numbers);
   }
   ```

3. **Ownership Error Fix:**
   Fix the following code with ownership errors.

   ```rust
   // Original code with errors
   fn main() {
       let s = String::from("hello");
       take_ownership(s);
       println!("{}", s); // Error: value used here after move
       
       let x = 5;
       make_copy(x);
       println!("{}", x); // This works fine
   }
   
   fn take_ownership(some_string: String) {
       println!("{}", some_string);
   }
   
   fn make_copy(some_integer: i32) {
       println!("{}", some_integer);
   }
   
   // Fixed code
   fn main() {
       let s = String::from("hello");
       let s = take_ownership_and_return(s); // Take and return ownership
       println!("{}", s); // Now it works
       
       // Or use borrowing
       let s2 = String::from("world");
       borrow_string(&s2); // Only borrow the string
       println!("{}", s2); // Works because s2 is still in scope
       
       let x = 5;
       make_copy(x);
       println!("{}", x); // This works fine because i32 implements Copy
   }
   
   fn take_ownership_and_return(some_string: String) -> String {
       println!("{}", some_string);
       some_string // Return ownership back
   }
   
   fn borrow_string(some_string: &String) {
       println!("{}", some_string);
   }
   
   fn make_copy(some_integer: i32) {
       println!("{}", some_integer);
   }
   ```

### Quiz:

1. **When a variable goes out of scope, what happens to the data it owns?**
   - A) It remains in memory until garbage collection
   - B) It is dropped and memory is freed immediately
   - C) It becomes inaccessible but stays in memory
   - D) It depends on the type of variable
   
   **Answer:** B) It is dropped and memory is freed immediately

2. **What is the difference between `&String` and `&mut String`?**
   - A) `&String` creates a copy, `&mut String` allows modification
   - B) `&String` is an immutable reference, `&mut String` is a mutable reference
   - C) `&String` works with any String, `&mut String` only works with strings declared as mutable
   - D) There is no difference
   
   **Answer:** B) `&String` is an immutable reference, `&mut String` is a mutable reference

3. **Can you have multiple immutable references to the same data at once?**
   - A) Yes, any number of immutable references are allowed
   - B) No, only one reference of any kind is allowed
   - C) Yes, but only if the data is on the stack
   - D) Yes, but only up to a maximum of 8 references
   
   **Answer:** A) Yes, any number of immutable references are allowed

4. **Why doesn't Rust allow multiple mutable references to the same data at the same time?**
   - A) To prevent memory leaks
   - B) To improve performance
   - C) To prevent data races
   - D) It's a limitation of the language
   
   **Answer:** C) To prevent data races