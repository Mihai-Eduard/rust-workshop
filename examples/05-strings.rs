use std::ops::Add;

fn main() {
    // Create a mutable String with `String::from(...)` and print it.
    let s = String::from("Hello");
    println!("{}", s);

    // Add ", World!" to the String and print it.
    let t = s.add(", World!");
    println!("{}", t);
}
