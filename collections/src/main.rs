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
}
