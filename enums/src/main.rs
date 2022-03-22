#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
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

    let message = Message::Quit;
    let message2 = Message::Move{ x:5, y:10 };
    let message3 = Message::Write(String::from("Test"));
    let message4 = Message::ChangeColor(32, 32, 32);

    let money = Coin::Penny;
    let value = value_in_cents(money);

    let money2 = Coin::Quarter(Province::Ns);
    let value2 = value_in_cents(money2);

    check(&Some(32));
    println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", 
    home, loopback, some_number, some_string, absent_number, 
    message, message3, message4, value, value2);

    match message2 {
        Message::Move { x, y } => {
            println!("{} {}", x, y)
        },
        communication => println!("{:?}", communication.call()),
    }

    let coin1 = Coin::Nickel;
    let coin2 = Coin::Loonie;
    let coin3 = Coin::Dime;

    value_in_cents(coin1);
    value_in_cents(coin2);
    value_in_cents(coin3);

    let money3 = Coin::Quarter(Province::Alb);
    let money4 = Coin::Quarter(Province::Bc);
    let money5 = Coin::Quarter(Province::Man);
    let money6 = Coin::Quarter(Province::Nb);
    let money7 = Coin::Quarter(Province::Nfld);
    let money8 = Coin::Quarter(Province::Nwt);
    let money9 = Coin::Quarter(Province::Ont);
    let money10 = Coin::Quarter(Province::Pei);
    let money11 = Coin::Quarter(Province::Que);
    let money12 = Coin::Quarter(Province::Sas);

    value_in_cents(money3);
    value_in_cents(money4);
    value_in_cents(money5);
    value_in_cents(money6);
    value_in_cents(money7);
    value_in_cents(money8);
    value_in_cents(money9);
    value_in_cents(money10);
    value_in_cents(money11);
    value_in_cents(money12);
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