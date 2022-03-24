// if let is just a more concise way of writing a match statement with
// a single conditional arm and a catch-all and do nothing arm

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
