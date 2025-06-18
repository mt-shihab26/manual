fn main() {
    let secret_number = stils::rand_range(1, 100);

    println!("Secret number: {}", secret_number);

    println!("Guess the number!\nPlease input the guess");

    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
