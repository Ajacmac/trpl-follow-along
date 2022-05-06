
// Panicing causing the program to immediately unwind
// Unwinding means walking immediately back up the stack
// and cleaning up the data from every function encountered

// Unwinding is a bit complicated and requires a significant
// amount of code to be included in the binary.
// The alternative is to simply abort on panic, which instead
// leaves all of the cleanup to the operating system to do.

use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    let f = File::open("hello.txt");

    // error handling with nested match statements
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error)
        }
    };

    // error handling using the _or_else method family
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // using expect to automatically pass the contents of Ok or 
    // panic with the provided message
    let mut f = File::open("hello.txt").expect("Failed to open hello.txt");


    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    };
}
