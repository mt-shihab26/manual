fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

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

pub fn scalar_types() {
    // integer data type
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

    // numeric_operations
    let sum = 5 + 10;
    println!("sum = {sum}");

    let difference = 95.5 - 4.3;
    println!("difference = {difference}");

    let quotient = 25.5 / 2.0;
    println!("quotient = {quotient}, type_of = {}", type_of(&quotient));
    let truncated = 25 / 2;
    println!("truncated = {truncated}, type_of = {}", type_of(&truncated));

    let reminder = 43 % 5;
    println!("reminder = {reminder}, type_of = {}", type_of(&reminder));

    // boolean_types
    let t = true;
    let f: bool = false; // explicit type annotation

    println!("t = {t}");
    println!("f = {f}");

    // char_types
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_chat = '😻';

    println!("c = {c}");
    println!("z = {z}");
    println!("heart_eyed_chat = {heart_eyed_chat}");
}

pub fn compound_types() {
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {tup:?}");

    let tup = (400, 5.5, 1);
    let (x, y, z) = tup;
    println!("x = {x}, y = {y}, z = {z}");
}

fn main() {
    // variable();
    // scalar_types();
    compound_types();
}
