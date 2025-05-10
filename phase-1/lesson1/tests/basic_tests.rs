// Basic tests for lesson1 concepts
#[test]
fn test_immutability() {
    let x = 5;
    // Uncommenting the line below would cause a compile-time error
    // x = 6;
    assert_eq!(x, 5);
}

#[test]
fn test_mutability() {
    let mut count = 0;
    count += 1;
    assert_eq!(count, 1);
}
