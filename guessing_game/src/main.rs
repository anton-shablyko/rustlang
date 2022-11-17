use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret Number is {secret_number}");

    loop {
        println!("Input your guess");

        // declare guess variable and read it from the user input
        // it will be stored as a string
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //convert "guess" String to u32 Integer
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // Ok(x) - instead of 'x' we can use any name e.g. `num`
            Ok(x) => x,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
