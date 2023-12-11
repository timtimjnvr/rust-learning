fn main() {
    // s is not valid here, it’s not yet declared
    let _s = "hello"; // s is valid from this point forward
                      // do stuff with s

    {
        // :: allows to namespace 'hello' under the String type
        // implements a request to the memory
        let mut s = String::from("hello");
        s.push_str(", world!"); // push_str() appends a literal to a String
        println!("{}", s);
    }
    // this scope is now over, and s is no longer valid
    // rust calls drop to return the memory used by s.

    // String = a pointer to the content of the string in memory, a length, and a capacity.
    // stored on the heap
    let s1 = String::from("hello");
    // get the same pointer to the content in memory, length and capacity.
    let _s2 = s1;
    // this leads to a problem because both s1 & s2 will be 'freed' from memory when exiting of the scope
    // to solve this double free error Rust drops s1 when s2 is created.
    // access of s1 here becomes illegal and generates an error at compile time.
    // Rust never does 'deep copy' of your data on the heap.

    // to deeply copy the heap data of a String you need to call clone
    // expensive !
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // copies on the stack (types of known size)
    // no difference between deep and shallow copy herer
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // FUNCTIONS
    {
        let s = String::from("hello"); // s comes into scope

        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here

        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    {
        let s1 = gives_ownership(); // gives_ownership moves its return
                                    // value into s1

        let s2 = String::from("world!"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
                                           // takes_and_gives_back, which also
                                           // moves its return value into s3
        println!("{s1}{s3}")
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello, "); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
