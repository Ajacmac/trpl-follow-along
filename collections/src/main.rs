#[derive(Debug)]
enum X {
    Int(i32),
    Float(f64),
}

fn main() {
    // Vectors are implemented using generics, so there has to be some
    // other indication of what type it is intended to contain, hence
    // the Vec<i32>
    let mut v: Vec<i32> = Vec::new();

    // Here the compiler interprets each of these as integers, and since
    // the default integer type is i32, this vector is the same type as 
    // the first one
    let v2 = vec![1, 2, 3];

    v.push(1);

    println!("{}", &v2[1]);
    // it turns out that this method of access, because it returns an option
    // cannot be used with the default formatter and instead requires debug
    println!("{:?}", v2.get(2));

    match v2.get(5) {
        Some(value) => println!("{}", value),
        None => println!("there's nothing there")
    };

    for i in 0..3 {
        v.push(i);
    }

    for i in &mut v {
        *i *= 40;
    }

    for i in &v {
        println!("{}", i);
    }

    let poly_vec = vec![
        X::Int(5),
        X::Float(5.5),
    ];

    println!("{:?}, {:?}", poly_vec[0], poly_vec[1]);

    // In Rust there's really only one type of string in the core language, the 
    // string slice. A string slice is a reference that points to some data stored 
    // elsewhere.

    // String literals are stored in the binary, and under the hood are treated 
    // exactly the same as any other string slice

    // Strings in Rust are implemented as byte collections, with the data itself
    // being UTF-8 encoded

    // String is owned
    // str is borrowed

    let mut s = String::new(); // owned string

    let s2 = "example text".to_string(); // owned string

    
}
