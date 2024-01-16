struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    // if the vector is empty, pop returns None, which stops the loop
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let v = vec!['a', 'b', 'c'];

    // tuple destructuration of the tuple returned by enumerate
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let point = (3, 5);
    // function parameters also uses pattern matching
    print_coordinates(&point);

    // pattern matching with variables
    let x = Some(5);
    let y = 10;

    // pattern shadowing (we need to use match guards if we want to match x == 10 in the second arm)
    match x {
        Some(50) | Some(10) => println!("Got 50"), // match multiple cases
        Some(y) => println!("Matched, y = {y}"), // this arm will match and y will be shadowed with the value 5 in the arm scope
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 5;

    match x {
        1..=5 => println!("one through five"), // match a range (only allowed for char and numeric values)
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    // struct destructuration
    // creates the variables a and b that match the values of the x and y fields of the p
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // or shorter (creates variables x & y)
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let s = Some(String::from("Hello!"));

    // s is moved into _s here
    if let Some(_s) = s {
        println!("found a string");
    }
    // this won't compile because s is not valide anymore
    // println!("{:?}", s);

    let s = Some(String::from("Hello!"));

    // because s is not moved into _, this code compiles
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point { x: 0, y: 0 };
    match origin {
        // ..  ignores any parts of a value that we haven’t explicitly matched
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    // same here
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let num = Some(4);
    // match guards
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // solves the pattern shadowing from earlier !
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
