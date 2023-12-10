use rand::Rng;

fn main() {
    // COMPOUND TYPES

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {y}");

    let first_number = tup.0;
    println!("The value of the first number is {first_number}");

    // empty tuple = unit
    // let empty_tuple = ();

    // array
    // array are allocated on the stack rather than the heap
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("first month is {}", months[0]);

    // create an array of i32 of size 5
    let _b: [i32; 5] = [1, 2, 3, 4, 5];

    // create an array of 5 elements with the same value
    let a = [3; 5];

    // rust memory safety principle happens at run time
    // this will compile and panic at runtime
    let index_out_bound = rand::thread_rng().gen_range(10..=11);
    println!("{}", a[index_out_bound]);
}
