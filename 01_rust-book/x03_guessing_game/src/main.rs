use std::cmp::Ordering::{Equal, Greater, Less};
use std::io::stdin;

fn main() {
    let secret_number: u32 = stils::rand_range(1, 100) as u32;

    println!("Guess the number!");

    loop {
        println!("Please input the guess");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess_number: u32 = guess.trim().parse().expect("Please type a number");

        match guess_number.cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
