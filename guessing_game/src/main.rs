use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the numer game!");

    // generate a random number in the range between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("Input your number: ");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        //try to cast string to integer
        // u32 - usigned 32 bit number.
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        // println!("\nYou guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess ({guess}) is too small!"),
            Ordering::Greater => println!("Your guess ({guess}) is too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("\nTry another number: ");
    }
}
