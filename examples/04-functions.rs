// 1. Define a function that takes a name as string literal as an argument and prints a greeting in your native language.
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 3. Define a function that takes a number as an argument and returns the square of that number.
fn sq(n: f32) -> f32 {
    n * n
}

fn main() {
    // 2. Call your greeting function with a string literal as an argument.
    let name = "ABC";
    greet(name);
    
    // 4. Call your square function with a number as an argument and print the result.
    println!("sq(5) = {}", sq(5.0));
}