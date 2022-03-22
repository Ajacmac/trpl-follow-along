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

#[derive(Debug)]
enum Province {
    Ns,
    Nb,
    Nfld,
    Pei,
    Ont,
    Que,
    Man,
    Sas,
    Alb,
    Bc,
    Nwt
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province),
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

    let money2 = Coin::Quarter(Province::Ns);
    let value2 = value_in_cents(money2);
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
        Coin::Quarter(province) => {
            println!("The province is {:?}", province);
            25
        },
        Coin::Loonie => 100,
    }
}