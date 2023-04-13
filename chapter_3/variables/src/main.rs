fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // Shadowing

    let x = 5;
    let x = x + 5;
    let x = x * 2;
    
    println!("The value of x is: {}", x);

    let space = "    ";
    let space = space.len();

    println!("The value of space is: {}", space);

    // i32 max size in rust
    let max = std::i32::MAX;

    println!("The value of max is: {}", max);

    // i32 lower size in rust
    let min = std::i32::MIN;

    println!("The value of min is: {}", min);

    let f: bool = false;
    println!("The value of f is: {}", f);


    // tuple

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {:?}", tup);
    println!("The value of tup is: {}", tup.0);

    let (x, y, z) = tup;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // Array

    let a = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);
    
    // Months Array
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    println!("The value of months is: {:?}", months);
}
