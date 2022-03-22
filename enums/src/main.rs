enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Loonie,
}

fn main() {

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let money = Coin::Penny;
    let value = value_in_cents(money);
}

fn check(optional: &Option<i32>) {
    match optional {
        Some(v) => println!("{}", v),
        None => println!("None!"),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("It's a penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Loonie => 100,
    }
}