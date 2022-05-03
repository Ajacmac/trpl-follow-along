
// Panicing causing the program to immediately unwind
// Unwinding means walking immediately back up the stack
// and cleaning up the data from every function encountered

// Unwinding is a bit complicated and requires a significant
// amount of code to be included in the binary.
// The alternative is to simply abort on panic, which instead
// leaves all of the cleanup to the operating system to do.

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file: {:?}", e),
            },
            other_error => panic!("Prolem opening the file {:?}", other_error)
        }
    };
}
