fn main() {
    // Declare a variable named `x` 

    // Create a scope and declare a variable named `y` in it. Print `x + y` from the scope.
    {

    };

    // Create a scope that returns `x + y` from it, assign it to a variable named `z` and print it.
    let x = 1;
    let z = {
        let y = 2;
        x + y
    };

    // Use if/else to print whether z is greater than 1 or not.
    if z >= 1 {
        println!("z is greater than 1");
    } else {
        println!("z is not greater than 1");
    }

    // Use a `loop` to print `z` 5 times.
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        println!("i = {}", i);
        i+=1;
    }

    // Use a while loop to print `z` 5 times (use an additional counter variable).
    while i < 5 {
        println!("i = {}", i);
        i+=1;
    }

    // Use if else to declare a variable that is true if `z` is greater than 1 and false otherwise.
    let z = if i > 5 {
        1
    } else {
        0
    };
    println!("z = {}", z);
    
    // Use a loop that returns the first randomly generated value that is greater than 0.5 (use the function `rand::random::<f64>()` to get a random float between 0 and 1).
    loop {
        let r = rand::random::<f64>();
        if r > 0.5 {
            println!("r = {}", r);
            break;
        }
    }

    // Use a for loop to print the numbers 1 to 10.
    for i in 1..10{
        println!("i = {}", i);
    }

    // Use a for loop to iterate throuth and array
    let arr = [1, 2, 3, 4, 5];
    for i in arr.iter() {
        println!("i = {}", i);
    }
}
