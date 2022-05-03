
// Panicing causing the program to immediately unwind
// Unwinding means walking immediately back up the stack
// and cleaning up the data from every function encountered

// Unwinding is a bit complicated and requires a significant
// amount of code to be included in the binary.
// The alternative is to simply abort on panic, which instead
// leaves all of the cleanup to the operating system to do.

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {:?}", error),
    };
}
