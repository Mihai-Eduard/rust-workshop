fn main() {
    // Declare a variable with each of these types: i32, u32, f64, bool, &str, and print them
    let x: i32 = 10;
    let y: u32 = 5;
    let z: f64 = 3.14;
    let b: bool = true;
    let s: &str = "Hello, world!";
    println!("x = {}, y = {}, z = {}, b = {}, s = {}", x, y, z, b, s);

    // Declare a variable named `x` that is the sum of two integers, and print it
    let i: i32 = x + y as i32;
    println!("i = {}", i);

    // Declare an array of i32 with 3 elements, and print it
    let arr: [i32; 3] = [1, 2, 3];

    // Print the array's 2nd element
    println!("arr[1] = {}", arr[1]);

    // Make the array mutable using shadowing, and change the 2nd element to 42
    let mut arr = arr;
    arr[1] = 5;
    println!("arr[1] = {}", arr[1]);
    
    // Declare a tuple with 3 elements: i32, &str, and f64, and print it
    let t: (i32, &str, f64) = (1, "abcdefghi", 3.14);

    // Print its 1st element
    println!("t.0 = {}", t.0);
}