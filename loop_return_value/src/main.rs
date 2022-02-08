fn main() {
    let mut counter = 0;
    let out = loop {
        counter += 1;
        if counter > 20 {
            break counter * 2;
        }
    };
    println!("output: {}", out);
}
