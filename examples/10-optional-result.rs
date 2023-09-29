fn find_nth_word(s: &str, n: usize) -> Option<&str> {
    let mut words = s.split_whitespace();
    let mut nth_word = None;
    for _ in 0..n {
        nth_word = words.next();
    }
    nth_word
}

fn main() {
    let word = find_nth_word("The quick brown fox", 3);
    match word {
        Some(w) => println!("The 3rd word in 'The quick brown fox' is '{}'.", w),
        None => println!("No 3rd word in 'The quick brown fox'."),
    }
}
