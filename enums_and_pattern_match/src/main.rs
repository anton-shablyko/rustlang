#[derive(Debug)]

// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let four = IpAddrKind::V4(String::from("127.0.0.1"));
//     let six = IpAddrKind::V6(String::from("::1"));

//     println!("{:?}", &four);
//     println!("{:?}", &six);

//     let some_number = Some(5);
//     let some_char = Some('e');

//     let absent_number: Option<i32> = None;
// }

// Only Quarters have states associated with some of the coins
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // add state Enum to Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let q = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(q));
}
