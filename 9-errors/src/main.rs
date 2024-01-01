use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, ErrorKind, Read};

// Box<dyn Error>> means “any kind of error.”
// when the main function returns a Result<T,E>, the executable will return 0 if Ok and a non zero value if main returns an Err (same as C)
fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // in C, this will cause a buffer overread (we will get whatever is at the location in memory that would correspond to that element in the data structure)
    // in Rust, it will stop execution and refuse to continue (no security vulnerability)
    // v[99];

    /*
    let _greeting_file_result = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
     */

    // cleaner way to do this with closures
    // unwrap_or_else returns the value inside Ok or calls the closure code in case of Err
    /*
       let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
     */

    // unwrap directly calls panic on a result.
    // let _greeting_file = File::open("other_file.txt").unwrap();

    // adding expect allows to add an error message and precise the intent in case of error
    // it unwraps the result if Ok
    // let _greeting_file = File::open("other_file.txt").expect("other_file.txt should be included in the project");

    let _greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    // ? operator allow to return the Err from the whole function or to continue if Ok
    // the ? operator returns the Err as an io::Error (converted)
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_much_simpler() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // next return the next line as a string (here it returns None if the line is empty)
    // the ? allows to returns in case of None
    text.lines().next()?.chars().last()
}
