fn main() {
    // one line conditions
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("{}", number);

    // loops
    loop {
        println!("again!");

        // go to the next iteration
        // continue;

        // exit the loop
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {result}");

    // loops labels
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        println!("remaining: {remaining}");

        loop {
            if remaining == 9 {
                break;
            }

            if count == 2 {
                // break of the parent loop
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut counter = 0;
    while counter != 3 {
        counter += 1;
    }

    println!("the counter is at {counter}");

    // looping on an array with for
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("25°C is {}°F", celsius_to_fahrenheit(25.0));

    for i in (0..10).into_iter() {
        println!("fibonnaci({i})={}", fibonnaci(i))
    }
}

fn celsius_to_fahrenheit(t: f32) -> f32 {
    (9.0 * t) / 5.0 + 12.0
}

fn fibonnaci(mut n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let mut _previous: i32 = 0;
    let mut next = 1;
    loop {
        if n == 0 {
            return next;
        }

        _previous = next;
        next = _previous + next;
        n -= 1;
    }
}
