fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const - always immutable
    // const - must be always anotated with the datatype
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    // shadowing
    let s = 5;
    let s = s + 1;
    {
        let s = s * 2;
        println!("The value of s in scope is: {s}");
    }

    println!("The value of s out of scope is: {s}");

    let spaces = "       ";
    let spaces = spaces.len();
    println!("{spaces}");
}
