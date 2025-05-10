# Rust Learning Path: Phase 1 - Fundamentals (2-3 weeks)

## Overview
In this first phase, you'll build a strong foundation in Rust's unique concepts. As an experienced software engineer, you'll find the syntax familiar, but Rust's memory safety guarantees without garbage collection will require new thinking patterns.

## Lesson 1: Getting Started with Rust

### Objectives:
- Install Rust toolchain
- Understand cargo basics
- Create your first program

### Key Concepts:
- Rustup & cargo workflow
- Basic syntax
- The `println!` macro

### Detailed Learning Plan:

#### 1. Setting Up Your Environment
```bash
# Install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# Setup an IDE (VSCode with rust-analyzer extension recommended)
```

#### 2. Understanding Cargo
- `cargo new` - Create a new project
- `cargo build` - Compile the project
- `cargo run` - Compile and run
- `cargo check` - Check for errors without compiling
- `cargo test` - Run tests
- Cargo.toml vs Cargo.lock

#### 3. Basic Syntax

**Sample Code:**
```rust
// Hello World in Rust
fn main() {
    println!("Hello, Rust!");
    
    // Variables and basic types
    let x: i32 = 42;  // Explicitly typed
    let y = 3.14;     // Type inference
    
    println!("x = {}, y = {}", x, y);
    
    // Immutability by default
    let name = "Rust";
    // name = "Java"; // This would cause a compilation error
    
    // Mutability requires 'mut'
    let mut count = 0;
    count += 1;
    println!("Count: {}", count);
}
```

### Practice Problems:

1. **Simple Calculator:**
   Create a program that takes two numbers and performs basic arithmetic operations (addition, subtraction, multiplication, division).

   ```rust
   fn main() {
       let a = 10;
       let b = 5;
       
       println!("Addition: {}", a + b);
       println!("Subtraction: {}", a - b);
       println!("Multiplication: {}", a * b);
       println!("Division: {}", a / b);
   }
   ```

2. **Temperature Converter:**
   Write a function that converts between Fahrenheit and Celsius.

   ```rust
   fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
       (fahrenheit - 32.0) * 5.0 / 9.0
   }
   
   fn celsius_to_fahrenheit(celsius: f64) -> f64 {
       celsius * 9.0 / 5.0 + 32.0
   }
   
   fn main() {
       let celsius = 25.0;
       let fahrenheit = celsius_to_fahrenheit(celsius);
       println!("{}°C = {}°F", celsius, fahrenheit);
       
       let fahrenheit = 98.6;
       let celsius = fahrenheit_to_celsius(fahrenheit);
       println!("{}°F = {}°C", fahrenheit, celsius);
   }
   ```

3. **Area Calculator:**
   Create a program that calculates the area of different shapes (rectangle, circle).

   ```rust
   fn rectangle_area(width: f64, height: f64) -> f64 {
       width * height
   }
   
   fn circle_area(radius: f64) -> f64 {
       std::f64::consts::PI * radius * radius
   }
   
   fn main() {
       let rect_width = 5.0;
       let rect_height = 10.0;
       println!("Rectangle area: {}", rectangle_area(rect_width, rect_height));
       
       let circle_radius = 7.0;
       println!("Circle area: {}", circle_area(circle_radius));
   }
   ```

### Quiz:

1. **What is the default mutability of variables in Rust?**
   - A) Mutable
   - B) Immutable
   - C) Depends on the type
   - D) Depends on the scope
   
   **Answer:** B) Immutable

2. **How do you make a variable mutable in Rust?**
   - A) Use the `mutable` keyword
   - B) Add the `mut` keyword before the variable name
   - C) Use the `var` keyword instead of `let`
   - D) Variables can't be mutable in Rust
   
   **Answer:** B) Add the `mut` keyword before the variable name

3. **What is the purpose of the `cargo` command?**
   - A) To compile Rust code
   - B) To run Rust programs
   - C) Rust's package manager and build system
   - D) To test Rust code
   
   **Answer:** C) Rust's package manager and build system

4. **What's the difference between `let` and `const` in Rust?**
   - A) `const` can be mutable, `let` cannot
   - B) `let` can be mutable, `const` cannot
   - C) `const` must be explicitly typed, `let` can use type inference
   - D) B and C are both correct
   
   **Answer:** D) B and C are both correct

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

## Lesson 3: Structs and Enums

### Objectives:
- Define and use structs
- Implement methods for structs
- Understand Rust's enum system

### Key Concepts:
- Struct definition and instantiation
- Field access
- Method implementation
- Enum variants and pattern matching

### Detailed Learning Plan:

#### 1. Working with Structs
- Defining structs
- Creating instances
- Field access and initialization
- Tuple structs and unit structs
- Struct update syntax

#### 2. Methods and Associated Functions
- Implementing methods for structs
- `self`, `&self`, and `&mut self`
- Multiple `impl` blocks
- Associated functions (like constructors)

#### 3. Enums and Pattern Matching
- Defining enums with different variants
- Enum values can contain data
- Option enum for null safety
- Pattern matching with `match`
- The `if let` construct

**Sample Code:**
```rust
// Struct definition
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing methods
impl Rectangle {
    // Constructor
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method that modifies self
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // Associated function (not a method)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Enum example
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Using structs
    let mut rect1 = Rectangle { width: 30, height: 50 };
    println!("Area: {}", rect1.area());
    
    rect1.double_size();
    println!("New area: {}", rect1.area());
    
    let square1 = Rectangle::square(25);
    println!("Square area: {}", square1.area());
    
    // Using enums
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Pattern matching with enums
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}
```

### Practice Problems:

1. **Car Struct:**
   Create a `Car` struct with properties and methods for a simple car simulation.

   ```rust
   struct Car {
       make: String,
       model: String,
       year: u32,
       fuel_level: f32, // 0.0 to 1.0
       mileage: u32,
   }
   
   impl Car {
       fn new(make: &str, model: &str, year: u32) -> Car {
           Car {
               make: make.to_string(),
               model: model.to_string(),
               year,
               fuel_level: 1.0, // Start with a full tank
               mileage: 0,
           }
       }
       
       fn drive(&mut self, distance: u32) -> bool {
           // Assume 1.0 fuel level can drive 500 miles
           let fuel_needed = distance as f32 / 500.0;
           
           if fuel_needed > self.fuel_level {
               println!("Not enough fuel to drive that far!");
               return false;
           }
           
           self.fuel_level -= fuel_needed;
           self.mileage += distance;
           println!("Drove {} miles. Fuel remaining: {:.1}%", 
                    distance, self.fuel_level * 100.0);
           true
       }
       
       fn refuel(&mut self) {
           self.fuel_level = 1.0;
           println!("Refueled to 100%");
       }
       
       fn info(&self) -> String {
           format!("{} {} ({}): {} miles, {:.1}% fuel", 
                   self.make, self.model, self.year, 
                   self.mileage, self.fuel_level * 100.0)
       }
   }
   
   fn main() {
       let mut my_car = Car::new("Toyota", "Corolla", 2020);
       println!("{}", my_car.info());
       
       my_car.drive(100);
       my_car.drive(200);
       println!("{}", my_car.info());
       
       my_car.refuel();
       println!("{}", my_car.info());
   }
   ```

2. **Shape Enum:**
   Implement a `Shape` enum with multiple variants and a method to calculate area.

   ```rust
   enum Shape {
       Circle(f64),             // Radius
       Rectangle(f64, f64),     // Width, Height
       Triangle(f64, f64, f64), // Three sides
   }
   
   impl Shape {
       fn area(&self) -> f64 {
           match self {
               Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
               Shape::Rectangle(width, height) => width * height,
               Shape::Triangle(a, b, c) => {
                   // Heron's formula
                   let s = (a + b + c) / 2.0;
                   (s * (s - a) * (s - b) * (s - c)).sqrt()
               }
           }
       }
       
       fn describe(&self) -> String {
           match self {
               Shape::Circle(radius) => format!("Circle with radius {}", radius),
               Shape::Rectangle(width, height) => format!("Rectangle {}x{}", width, height),
               Shape::Triangle(a, b, c) => format!("Triangle with sides {}, {}, {}", a, b, c),
           }
       }
   }
   
   fn main() {
       let shapes = vec![
           Shape::Circle(5.0),
           Shape::Rectangle(4.0, 6.0),
           Shape::Triangle(3.0, 4.0, 5.0),
       ];
       
       for shape in shapes {
           println!("{}: area = {:.2}", shape.describe(), shape.area());
       }
   }
   ```

3. **Inventory Management:**
   Build a simple inventory management system using structs and enums.

   ```rust
   enum ProductCategory {
       Electronics,
       Clothing,
       Food,
       Books,
   }
   
   struct Product {
       id: u32,
       name: String,
       price: f64,
       category: ProductCategory,
       stock: u32,
   }
   
   struct Inventory {
       products: Vec<Product>,
   }
   
   impl Inventory {
       fn new() -> Inventory {
           Inventory { products: Vec::new() }
       }
       
       fn add_product(&mut self, product: Product) {
           self.products.push(product);
       }
       
       fn find_product(&self, id: u32) -> Option<&Product> {
           self.products.iter().find(|p| p.id == id)
       }
       
       fn update_stock(&mut self, id: u32, new_stock: u32) -> bool {
           for product in &mut self.products {
               if product.id == id {
                   product.stock = new_stock;
                   return true;
               }
           }
           false
       }
       
       fn products_by_category(&self, category: &ProductCategory) -> Vec<&Product> {
           self.products.iter()
               .filter(|p| std::mem::discriminant(&p.category) == std::mem::discriminant(category))
               .collect()
       }
       
       fn total_value(&self) -> f64 {
           self.products.iter()
               .map(|p| p.price * p.stock as f64)
               .sum()
       }
   }
   
   fn main() {
       let mut inventory = Inventory::new();
       
       inventory.add_product(Product {
           id: 1,
           name: String::from("Laptop"),
           price: 999.99,
           category: ProductCategory::Electronics,
           stock: 10,
       });
       
       inventory.add_product(Product {
           id: 2,
           name: String::from("T-shirt"),
           price: 19.99,
           category: ProductCategory::Clothing,
           stock: 50,
       });
       
       inventory.add_product(Product {
           id: 3,
           name: String::from("Apples (1kg)"),
           price: 3.99,
           category: ProductCategory::Food,
           stock: 100,
       });
       
       // Update stock
       inventory.update_stock(1, 8);
       
       // Find a product
       if let Some(product) = inventory.find_product(2) {
           println!("Found: {} - ${} ({} in stock)", 
                   product.name, product.price, product.stock);
       }
       
       // List electronics
       println!("Electronics:");
       for product in inventory.products_by_category(&ProductCategory::Electronics) {
           println!("- {} (${:.2})", product.name, product.price);
       }
       
       // Total inventory value
       println!("Total inventory value: ${:.2}", inventory.total_value());
   }
   ```

### Quiz:

1. **What is the difference between a struct method and an associated function?**
   - A) Methods can be called, associated functions cannot
   - B) Methods take `self` as their first parameter, associated functions don't
   - C) Methods are called on instances, associated functions are called on the type
   - D) B and C are both correct
   
   **Answer:** D) B and C are both correct

2. **How do you define a method that can modify a struct instance?**
   - A) Use `self` as the first parameter
   - B) Use `&self` as the first parameter
   - C) Use `&mut self` as the first parameter
   - D) Use `mut self` as the first parameter
   
   **Answer:** C) Use `&mut self` as the first parameter

3. **What's the purpose of the `impl` keyword?**
   - A) To implement a trait for a type
   - B) To define methods associated with a type
   - C) To import functionality from another module
   - D) Both A and B
   
   **Answer:** D) Both A and B

4. **How does Rust's enum differ from enums in other languages?**
   - A) Rust enums can have methods
   - B) Rust enum variants can contain data
   - C) Rust enums can implement traits
   - D) All of the above
   
   **Answer:** D) All of the above

## Lesson 4: Error Handling

### Objectives:
- Handle errors with Result and Option
- Understand panic vs. recoverable errors
- Use error propagation

### Key Concepts:
- Option<T> for optional values
- Result<T, E> for error handling
- The `?` operator
- Custom error types

### Detailed Learning Plan:

#### 1. Options and Null Safety
- Using `Option<T>` instead of null
- Pattern matching with Option
- Unwrapping options safely
- The `?` operator with Options

#### 2. Result for Error Handling
- Using `Result<T, E>` for operations that can fail
- Handling errors with `match`
- Propagating errors with `?`
- Converting between different error types

#### 3. Panic and When to Use It
- Unrecoverable errors with `panic!`
- `unwrap` and `expect` methods
- Recoverable vs. unrecoverable errors
- Custom error types

**Sample Code:**
```rust
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // Option example
    let optional_value: Option<i32> = Some(42);
    
    // Using match with Option
    match optional_value {
        Some(value) => println!("Got a value: {}", value),
        None => println!("No value found"),
    }
    
    // Using if let for concise Option handling
    if let Some(value) = optional_value {
        println!("Value: {}", value);
    }
    
    // Result example - handling potential file error
    let file_result = File::open("hello.txt");
    
    // Using match with Result
    let file = match file_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem opening the file: {:?}", error);
            return;
        }
    };
    
    // Calling a function that returns a Result
    match read_file_contents("config.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(error) => println!("Error reading file: {:?}", error),
    }
    
    // Using the ? operator with Result
    let result = read_username_from_file();
    println!("Username result: {:?}", result);
}

// Function that returns a Result
fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // ? operator returns early on Err
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// More concise with ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

### Practice Problems:

1. **Safe String to Integer Conversion:**
   Implement a function that safely converts a string to an integer.

   ```rust
   fn parse_int(s: &str) -> Result<i32, String> {
       match s.parse::<i32>() {
           Ok(num) => Ok(num),
           Err(_) => Err(format!("Failed to parse '{}' as integer", s)),
       }
   }
   
   fn main() {
       let test_cases = vec!["42", "-5", "3.14", "hello"];
       
       for test in test_cases {
           match parse_int(test) {
               Ok(num) => println!("'{}' parsed as: {}", test, num),
               Err(e) => println!("Error: {}", e),
           }
       }
   }
   ```

2. **Multiple File Reader:**
   Create a program that reads multiple files, handling errors appropriately.

   ```rust
   use std::fs::File;
   use std::io::{self, Read};
   use std::path::Path;
   
   fn read_file(path: &str) -> Result<String, io::Error> {
       let mut file = File::open(path)?;
       let mut contents = String::new();
       file.read_to_string(&mut contents)?;
       Ok(contents)
   }
   
   fn read_multiple_files(paths: &[&str]) -> Vec<(String, Result<String, io::Error>)> {
       let mut results = Vec::new();
       
       for path in paths {
           let result = read_file(path);
           results.push((path.to_string(), result));
       }
       
       results
   }
   
   fn main() {
       let files = ["file1.txt", "file2.txt", "nonexistent.txt"];
       let results = read_multiple_files(&files);
       
       for (path, result) in results {
           match result {
               Ok(contents) => println!("File '{}' contains {} bytes", path, contents.len()),
               Err(e) => println!("Error reading '{}': {}", path, e),
           }
       }
   }
   ```

3. **Custom Error Type:**
   Design a custom error type for a domain-specific application.

   ```rust
   use std::fmt;
   use std::error::Error;
   use std::io;
   use std::num::ParseIntError;
   
   // Define our custom error type
   #[derive(Debug)]
   enum ConfigError {
       IoError(io::Error),
       ParseError(ParseIntError),
       MissingField(String),
       InvalidValue(String),
   }
   
   // Implement Display for our error
   impl fmt::Display for ConfigError {
       fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match self {
               ConfigError::IoError(err) => write!(f, "IO error: {}", err),
               ConfigError::ParseError(err) => write!(f, "Parse error: {}", err),
               ConfigError::MissingField(field) => write!(f, "Missing field: {}", field),
               ConfigError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
           }
       }
   }
   
   // Implement the Error trait
   impl Error for ConfigError {
       fn source(&self) -> Option<&(dyn Error + 'static)> {
           match self {
               ConfigError::IoError(err) => Some(err),
               ConfigError::ParseError(err) => Some(err),
               _ => None,
           }
       }
   }
   
   // Implement From for automatic conversion
   impl From<io::Error> for ConfigError {
       fn from(err: io::Error) -> Self {
           ConfigError::IoError(err)
       }
   }
   
   impl From<ParseIntError> for ConfigError {
       fn from(err: ParseIntError) -> Self {
           ConfigError::ParseError(err)
       }
   }
   
   // Our config structure
   struct Config {
       port: u16,
       hostname: String,
       max_connections: u32,
   }
   
   // Function to parse a config file
   fn parse_config(path: &str) -> Result<Config, ConfigError> {
       // Read the file
       let contents = std::fs::read_to_string(path)?;
       
       let mut port = None;
       let mut hostname = None;
       let mut max_connections = None;
       
       // Parse each line
       for line in contents.lines() {
           let parts: Vec<&str> = line.splitn(2, '=').collect();
           if parts.len() != 2 {
               continue;
           }
           
           let key = parts[0].trim();
           let value = parts[1].trim();
           
           match key {
               "port" => {
                   let port_value: u16 = value.parse()?;
                   if port_value < 1024 {
                       return Err(ConfigError::InvalidValue(
                           "Port must be above 1023".to_string()
                       ));
                   }
                   port = Some(port_value);
               },
               "hostname" => {
                   hostname = Some(value.to_string());
               },
               "max_connections" => {
                   max_connections = Some(value.parse()?);
               },
               _ => {} // Ignore unknown keys
           }
       }