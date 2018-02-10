fn main() {
    let x = "Hello"; // &str
    print!("{}", x);

    let mut y = " World".to_string(); // String
    print!("{}", y);

    y.push_str("!"); // can push_str because mut variable.

    let z = &y[y.len() - 1 .. y.len()]; // borrow variable y and slicing.

    print!("{}", z); // "!"
}