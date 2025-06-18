mod random;

fn main() {
    for _ in 0..100 {
        let result = random::rand();
        println!("{}", result);
    }

    // let secret_number = random::rand();
    // println!("Secret number: {}", secret_number);
    //
    // println!("Guess the number!\nPlease input the guess");
    //
    // let mut guess = String::new();
    // std::io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");
    //
    // println!("You guessed: {}", guess);
}
