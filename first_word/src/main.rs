fn main() {
    let test1 = "test test test";
    println!("{}", find_first_no_slices(&test1));
}

fn find_first_no_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}