use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    //create a secret number

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secret is:  {secret_number}");

    loop {
        println!("Input your number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } // prompt user to guess a number
              // compare secret number against the guess and return hints for the user
        }
    }
}
