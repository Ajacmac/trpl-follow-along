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
}
