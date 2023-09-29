fn main() {
    // Declare a variable
    let x: i32 = 10;

    // Print its value 
    println!("x = {}", x);

    // Try changing the variable

    // Make it mutable
    let mut y: i32 = 14;
    println!("y = {}", y);

    // Try changing the variable again
    y = 15;
    println!("y = {}", y);

    // Try shadowing the variable
    let mut x: i32 = 14;
    println!("x = {}", x);
    x = 10;
    println!("x = {}", x);
}
