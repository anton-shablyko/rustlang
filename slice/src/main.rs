// take a string with words separated by space and return a first word

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (n, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..n];
        }
    }
    &s[..]
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);
    s.clear();

    let s2 = String::from("hello world 2");

    let hello = &s2[..5];
    let world = &s2[6..];
    println!("{}", hello);
    println!("{}", world);
}
