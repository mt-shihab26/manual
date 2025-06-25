pub fn variable() {
    // variables //
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing //
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len(); // changing type with shadowing
    println!("The length of spaces is: {spaces}");
}

pub fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {guess}");

    // integer literals
    let decimal = 123_45;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte_u8_only = b'A';

    println!("decimal = {decimal}");
    println!("hex = {hex}");
    println!("octal = {octal}");
    println!("binary = {binary}");
    println!("byte_u8_only = {byte_u8_only}");

    // floating point data type
    let x = 2.0; // f64;
    let y: f32 = 3.0;

    println!("x = {x}");
    println!("y = {y}");
}

fn main() {
    // variable();
    data_types();
}
