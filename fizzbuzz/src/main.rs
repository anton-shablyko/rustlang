use std::io;

fn main() {
    println!("Welcome to FizzBuzz!!\nEnter your number...");

    // get user's input as a string and convert it to u32
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: u32 = x.trim().parse().expect("Couldn't parse the number");

    for n in 1..x + 1 {
        let mut r = "nope";

        if n % 3 == 0 {
            r = "fizz"
        }
        if n % 5 == 0 {
            r = "buzz"
        }
        if (n % 3 == 0) && (n % 5 == 0) {
            r = "fizzbuzz"
        }

        println!("{n}: {r}");
    }
}
