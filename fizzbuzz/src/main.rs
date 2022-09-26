use std::io;

fn main() {
    //my first attempt at fizzbuzz
    println!("Welcome to FizzBuzz!!\nEnter your number...");

    // get user's input as a string and convert it to u32
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read line");
    let x: u32 = x.trim().parse().expect("Couldn't parse the number");
    // basic(x)
    using_match(x);
}

fn basic(user_input: u32) {
    for n in 1..=user_input {
        let r;

        if (n % 3 == 0) && (n % 5 == 0) {
            r = "fizzbuzz"
        } else if n % 3 == 0 {
            r = "fizz"
        } else if n % 5 == 0 {
            r = "buzz"
        } else {
            r = "nope"
        }

        println!("{n}: {r}");
    }
}

fn using_match(user_input: u32) {
    for x in 1..=user_input {
        match (x % 3, x % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{x}"),
        }
    }
}
// TODO: move user inputfunctiuonalityu into a separate function
