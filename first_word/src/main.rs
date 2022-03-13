fn main() {
    let test1 = String::from("test test test");
    println!("{}", find_first_no_slices(&test1));
}

fn find_first_no_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}