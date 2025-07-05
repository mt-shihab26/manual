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
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LEFTOFF!!!")
}

pub fn foreach_loop() {
    let elements = [10, 20, 30, 50];

    for element in elements {
        println!("the value is: {element}");
    }
}

pub fn for_loop_countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    // dicision();
    // repetation();
    // loop_label();
    // while_loop();
    // foreach_loop();
    for_loop_countdown();
}
