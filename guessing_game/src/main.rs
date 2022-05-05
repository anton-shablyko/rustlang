use std::io;

fn main() {
    println!("Guess the numer game!");
    println!("Enter your number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {}", guess);
}
