fn main() {
    let greeting = "Hello, Rust!";
    let mut counter = 5;
    counter += 1;
    println!("{}", greeting);

    //create variable using type inference
    let x = 42;
    let y: f64 = 3.14;
    println!("x = {}, y = {}", x, y);

    let mut num = 10;
    num = num + 5;
    num = num * 2;
    println!("num = {}", num);

    let name = "Caleb";
    let age = 30;
    println!("{},{}", name, age);


    let tempc = 1.0;
    let tempf = (tempc * 9.0/5.0) + 32.0;
    println!("f:{} c:{}", tempf, tempc);
}
