#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Rectange area is {}", rect.area());

    let rect2 = Rectangle { width: 20, ..rect };
    println!("{}", rect2.area());
}
