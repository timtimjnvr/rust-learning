use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // in C, this will cause a buffer overread (we will get whatever is at the location in memory that would correspond to that element in the data structure)
    // in Rust, it will stop execution and refuse to continue (no security vulnerability)
    // v[99];

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

    // cleaner way to do this with closures
    // unwrap_or_else returns the value inside Ok or calls the closure code in case of Err
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // unwrap directly calls panic on a result.
    // let _greeting_file = File::open("other_file.txt").unwrap();

    // adding expect allows to add an error message and precise the intent in case of error
    // it unwraps the result if Ok
    let _greeting_file =
        File::open("other_file.txt").expect("other_file.txt should be included in the project");
}
