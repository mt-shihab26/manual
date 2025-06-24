fn variable() {
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

fn data_types() {
    let guess = "42".parse().expect("Not a number!");
}

fn main() {
    // variable();
    data_types();
}
