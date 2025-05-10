fn main() {
    // Hello World example
    println!("Hello, Rust!");
    
    // Variables and basic types
    let x: i32 = 42;  // Explicitly typed
    let y = 3.14;     // Type inference
    
    println!("x = {}, y = {}", x, y);
    
    // Immutability demonstration
    let name = "Rust";
    println!("Learning {}", name);
    
    // Mutability example
    let mut count = 0;
    count += 1;
    println!("Count is now: {}", count);
}
