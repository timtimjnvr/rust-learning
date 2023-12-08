use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    // mut -> means the variable is modifiable
    // without mut a variable value can't be modified
    // String::new() creates a growable instance of string
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
