fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("This is a");
    let len = calculate_length(&s1);

    println!("The lenght of '{s1}' is {len}");
    change(&mut s2);

    println!("{s2}")
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// how to use a mutable references
fn change(s: &mut String) {
    s.push_str(" test");
}
