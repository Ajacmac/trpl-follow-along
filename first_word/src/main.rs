fn main() {
    let test1 = String::from("test test test");
    println!("{}", find_first_no_slices(&test1));
}

fn find_first_no_slices(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}