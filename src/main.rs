use std::io;
// the above are called prelude

fn main() {
    println!("Guess the number!");

    println!("Please input your guess!");

    // Rust !== JavaScript
    // let a === const a 
    // let mut a === let a
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}