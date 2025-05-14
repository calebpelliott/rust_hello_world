fn main() {
    // Memory allocation demonstration
    println!("
=== Memory Growth Strategy ===>");
    let mut s = String::from("hi");
    println!("Initial capacity: {}", s.capacity());
    
    // Track reallocations
    let mut last_capacity = s.capacity();
    for i in 0..10 {
        s.push_str("hello");  // Add 5 chars each time
        let current_capacity = s.capacity();
        
        if current_capacity != last_capacity {
            println!("Reallocation occurred at length {}", s.len());
            println!("New capacity: {}", current_capacity);
            last_capacity = current_capacity;
        }
    }
    
    // Show that we can also pre-allocate if we know the size
    println!("
=== Pre-allocation Example ===>");
    let mut pre_allocated = String::with_capacity(50);
    println!("Pre-allocated capacity: {}", pre_allocated.capacity());
    pre_allocated.push_str("hello world");
    println!("Length: {}, Capacity still: {}", 
             pre_allocated.len(), 
             pre_allocated.capacity());

    println!("
=== Original Examples ===>");


    // Demonstrating growable Strings
    println!("
=== Growable String Example ===");
    let mut growable = String::from("Hello");
    println!("Original string: {}", growable);
    println!("Original capacity: {}", growable.capacity());
    
    // We can add more content
    growable.push_str(", World");  // Append a string slice
    println!("After push_str: {}", growable);
    
    growable.push('!');  // Append a single character
    println!("After push: {}", growable);
    
    // Show how capacity grows
    println!("Final capacity: {}", growable.capacity());
    
    // String literals (not growable)
    let literal = "Hello";
    // literal.push_str(", World");  // This would not compile - string literals are not growable
    
    println!("
=== Original Examples ===");


    // 1. Basic Ownership Example
    println!("1. Basic Ownership Example:");
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1;  // ownership moves to s2
    // println!("{}", s1);  // This would not compile - s1 no longer owns the value
    println!("s2: {}", s2);  // This works fine

    // 2. Copy Types Example
    println!("\n2. Copy Types Example:");
    let x = 5;  // integers are Copy
    let y = x;  // x is copied to y
    println!("x: {}, y: {}", x, y);  // Both work because integers implement Copy

    // 3. Cloning Example
    println!("\n3. Cloning Example:");
    let s3 = String::from("hello");
    let s4 = s3.clone();  // Creates a deep copy
    println!("s3: {}, s4: {}", s3, s4);  // Both work because we cloned

    // 4. Ownership with Functions
    println!("\n4. Ownership with Functions:");
    let s5 = String::from("hello");
    takes_ownership(s5);
    // println!("{}", s5);  // Would not compile - ownership moved to function

    let x = 5;
    makes_copy(x);
    println!("x still available: {}", x);  // Works because i32 implements Copy

    // 5. References and Borrowing
    println!("\n5. References and Borrowing:");
    let mut s6 = String::from("hello");
    let len = calculate_length(&s6);  // Immutable borrow
    println!("Length of '{}' is {}", s6, len);

    change_string(&mut s6);  // Mutable borrow
    println!("Modified string: {}", s6);

    // 6. Multiple References Example
    println!("\n6. Multiple References Example:");
    let mut s7 = String::from("hello");
    {
        let r1 = &s7;  // First immutable reference
        let r2 = &s7;  // Second immutable reference - OK
        println!("r1: {}, r2: {}", r1, r2);
    }  // r1 and r2 go out of scope

    let r3 = &mut s7;  // Now we can have a mutable reference
    r3.push_str(" world");
    println!("Modified s7: {}", r3);
}

// Function that takes ownership
fn takes_ownership(some_string: String) {
    println!("Took ownership of: {}", some_string);
}  // some_string goes out of scope and is dropped

// Function that takes a copy
fn makes_copy(some_integer: i32) {
    println!("Made a copy of: {}", some_integer);
}  // some_integer goes out of scope but nothing special happens

// Function that borrows a reference
fn calculate_length(s: &String) -> usize {
    s.len()
}  // s goes out of scope but doesn't drop what it refers to

// Function that borrows a mutable reference
fn change_string(s: &mut String) {
    s.push_str(" world");
}
