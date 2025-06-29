pub fn dicision() {
    let number = 3;

    if number < 5 {
        println!("condition was true")
    } else if number < 10 {
        println!("less then 10")
    } else {
        println!("condition was false")
    }

    // if number {
    //     println!("This is error");
    // }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

pub fn repetation() {
    // loop {
    //     println!("agian!");
    // }

    // return values from loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");
}

pub fn loop_label() {
    //
}

fn main() {
    // dicision();
    // repetation();
    loop_label();
}
