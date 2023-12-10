fn main() {
    // Expressions
    // the code block inside {} is an expression which evaluates to x+1
    let y = {
        let x = 3;
        // there no ';' at the end of a line -> it's an expression
        // adding a ';' turns the expression into a statement
        x + 1
    };

    println!("The value of y is: {y}");

    // functions which return values : type must be indicated with '-> <type>'
    // returned values is synonym with the latest value reached in the function body
    // can use return to exit a function
    fn five() -> i32 {
        5
    }

    let v = five();
    println!("The value of v is: {v}");
    println!("{}", plus_one(5));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
