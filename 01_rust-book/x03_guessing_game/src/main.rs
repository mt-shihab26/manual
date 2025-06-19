use rand::Rng;

fn main() {
    // let secret_number: u32 = stils::rand_range(1, 100) as u32;
    let secret_number: u32 = rand::rng().random_range(1..=100);

    // println!("Secret Number: {}", secret_number);

    println!("Guess the number!");

    loop {
        println!("Please input the guess");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Type valid number");
                continue;
            }
        };

        match guess_number.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
