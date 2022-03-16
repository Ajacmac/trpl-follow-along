#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "Rect1 is {:#?}.",
        rect1
    );

    dbg!(&rect1);

    println!(
        "The area of rect1 is {:#?}.",
        area(&rect1)
    );
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}