
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

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // first potential return
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // second potential return
    }

    // both potential error return values are the same type of error
    // different error types as return values would need more careful
    // handling in order to work
    // the ? operator can avoid this problem since it automatically
    // converts the passed in error value to the appropriate type
    // of return value
}

// the ?'s here will either pass the value they receive or, if that
// value is an error, will cause the function to return that value
// providing a convenient way to handle error propagation
// the ? operator tells the compiler to check the return type
// of the containing function and the ? operator will convert
// received error values if necessary so they'll match the error
// return type
fn concise_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// this example includes method chaining off ?
// method chaining like this makes for very elegant code
fn terse_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// the ? can also be used on an Option(), not just a Result()
// ? will not convert between Option and Result, so the return
// type of the function needs to match the type ? is being given
// you can, however, convert between Option and Result with ok()
// and ok_or()

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()

    // this takes a &str, converts it to lines
    // grabs the first line, which can be empty
    // passes the option to ?
    // which passes that to chars() if there is a line
    // and finally last() will return the final character
}