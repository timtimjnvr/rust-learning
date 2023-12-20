use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");
    // the range specified : start..=end, it's inclusive of start and end
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess.");

        // mut -> means the variable is modifiable
        // without mut a variable value can't be modified
        // String::new() creates a growable instance of string
        let mut guess: String = String::new();

        io::stdin()
            // by default reference are immutable
            // adding mut allows the reference to be mutable
            .read_line(&mut guess)
            // Return a result (Ok, Err)
            // had an expect method
            // if the Result is an Err the program will crash with the given msg
            .expect("Failed to read line");

        // convert to number
        // trim eliminates \n
        // parse converts a string to another type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // cmp compare two values
        // and returns a variant of Ordering
        // Ordering is an enum with values Less, Greater, Equal
        // match is use to decide what to do next based on which variant of Ordering was returned
        // from cmp with the values guess and secret_number
        match guess.cmp(&secret_number) {
            // arms of the math expression associates a value to a piece of code
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
