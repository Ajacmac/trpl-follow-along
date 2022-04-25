use std::collections::HashMap;

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

    s.push_str("test"); // push_str takes a string slice so ownership isn't taken
    println!("{}", s);

    // using the concatenation operator + would work, but that operator takes
    // ownership of the value on the left, so the results can be confusing if 
    // you aren't careful
    // eg: s + &s2      in this case ownership of s gets passed into +
    // note: simply using a str as the first argument isn't allowed, so won't work

    s = s2 + &s;

    println!("{}", s);
    // println!("{}", s2); // errors because this memory was freed

    let s3 = String::from("riddle");
    let s4 = String::from("me");
    let s5 = String::from("this");

    // this format macro allows for pretty elegant string construction
    // a particular advantage is that you that use {:?} for types that
    // don't implement display, but do implement debug
    let s6 = format!("{} {} {}", s3, s4, s5);

    // chars() returns valid utf-8 characters, but that's not the same
    // as a grapheme cluster, where a diacritic is added to a character
    // to create a new character. Grapheme clusters will be split into
    // their component pieces and printed separately.
    for c in s6.chars() {
        println!("{}", c);
    }

    // Hash maps are dictionaries, and allow you to access values with a
    // key that can be any type, not just an int.

    // within a hashmap, all of the keys must be the same type, and all
    // the values must be the same type

    // hashmaps are stored on the heap. they have to be since they must
    // be able to support resizing, etc. once there are a certain number
    // of hash collisions

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let a = String::from("a");
    let b = String::from("b");

    let mut map = HashMap::new();
    map.insert(a, b);

    println!("{:?}", map);
    // println!("{}", a); // errors since inserting into the map passes ownership
    if let Some(val) = map.get("a") { 
        println!("the value is {}", val) 
    }

    match map.get("a") {
        Some(val) => println!("{:?}", val),
        None => println!("none"),
    }

    for (key, value) in &map {
        println!("{}", value);
    }

    // blind overwrite of a value
    map.insert(String::from("a"), String::from("c"));
    println!("value after overwrite {}", map.get("a").unwrap());

    // entering a new value only if one doesn't exist for the key
    map.entry(String::from("b")).or_insert(String::from("b"));
    // won't overwrite
    // returns a mutable reference, either to the pre-existing value
    // or the new value for the key, what we passed into or_insert()
    map.entry(String::from("b")).or_insert(String::from("bad data"));

    println!("{:?}", map);

    // updating a value conditionally based on current value
    // first cloning map because I need a mutable borrow here
    // and there's already an immutable one above
    let mut map2 = map.clone();
    for item in &map {
        let a = map2.entry(String::from("a")).or_insert(String::from("a"));
        *a = format!("{}{}", a, String::from("a"));
    }
    println!("{:?}", map2);

    // another similar kind of example showing math operations in place
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 0);
    scores.insert(String::from("Blue"), 0);

    let red_score = scores.entry(String::from("Red")).or_insert(0);
    *red_score += 1;


    // pig latin example
    let input = String::from("This is a sentence I want converted to pig latin");
    let mut output = String::new();

    // single quotes are for characters, double quotes
    // won't work
    let mut temp: char = ' ';

    for word in input.split_whitespace() {
        temp = ' ';
        if word.len() > 1 {
            for c in word.chars() {
                if temp == ' ' {
                    temp = c.to_ascii_lowercase();
                } else {
                    output += &format!("{}", c);
                }
            }
            output += &format!("{}ay ", temp);
        }
    }

    output = format!("{}{}", &output[..1].to_ascii_uppercase(), &output[1..]);

    println!("{}", output);
}
