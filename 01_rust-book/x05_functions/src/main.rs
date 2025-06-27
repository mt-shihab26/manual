fn main() {
    another_function(5);

    let y = {
        let x = 6;
        x + 1
    };

    println!("The value of y is: {y}");

    let sum = add(21, 5);
    println!("The sum of 21 and 5 is: {sum}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}")
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
