// Define a basic enum and an enum with associated data
enum Command{
    Quit,
    Move {x: f32, y: f32},
}
fn main() {
    // Use the enums
    let command1 = Command::Move {x: 1.0, y: 2.0};
    let command2 = Command::Quit;

    match command1 {
        Command::Quit => println!("Quit"),
        Command::Move {x, y} => println!("Move to {}, {}", x, y),
    }
    match command2 {
        Command::Quit => println!("Quit"),
        Command::Move {x, y} => println!("Move to {}, {}", x, y),
    }
}
