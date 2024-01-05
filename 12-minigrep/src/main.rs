use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).expect("The program needs at least 2 arguments");
    let file_path = args.get(2).expect("The program needs at least 2 arguments");

    let contents = fs::read_to_string(file_path).expect(&format!(
        "Should have been able to read the file located at {}",
        file_path,
    ));

    println!("With text:\n{contents}");
}
