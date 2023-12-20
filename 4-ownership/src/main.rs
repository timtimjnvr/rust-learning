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

    let len2 = calculate_length_with_pointer(&s2);
    println!("The length of '{}' is {}.", s2, len2);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("{r1}");
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;

    let mut s = String::from("hello");
    println!("{s}");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG PROBLEM because reference of s is used by r1 & r2
    println!("{}{}", r1, r2);
    let r3 = &s; // no problem
    let r4 = &s; // no problem

    println!("{} and {}", r3, r4);
    // variables r1 and r2 will not be used after this point (popped of the stack)

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // SLICE type

    let first_space_index = first_word(&s);
    println!("space index is {first_space_index}");

    let _word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];
    println!("{_hello}#{}", _hello.len());
    println!("{_world}#{}", _world.len());

    let s = String::from("hello");

    let _slice = &s[0..2]; // 'he'
    let _slice = &s[..2]; // 'he'

    let _len = s.len();
    let _slice = &s[3.._len]; // llo
    let _slice = &s[3..]; // llo

    let mut _s: String = String::from("hello world");
    let _word = first_word_v1(&s);
    // _s.clear(); // error ! because Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time
    // println!("the first word is: {}", _word);

    // this is an immutable string (stored in binary)
    // &str is an immutable reference
    let _s = "Hello, world!";

    let my_string = String::from("hello world");

    // deref coercions
    let _word = first_word_v2(&my_string[0..6]);
    let _word = first_word_v2(&my_string[..]);

    // works on references to `String`s, equivalent to whole slices of `String`s
    let _word = first_word_v2(&my_string);

    let my_string_literal = "hello world";

    // works on slices of string literals, whether partial or whole
    let _word = first_word_v2(&my_string_literal[0..6]);
    let _word = first_word_v2(&my_string_literal[..]);

    // String literals *are* string slices already
    // this works too, without the slice syntax!
    let _word = first_word_v2(my_string_literal);
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

fn calculate_length_with_pointer(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// takes a string by reference because we don't want to take ownership of it.
// firstly only return the index of the first space
fn first_word(s: &String) -> usize {
    // because we need to browse the string letter by letter.
    let bytes = s.as_bytes();
    // enumerate returns a tuple so we can destructure the tuple
    // enumerates returns a refernce so &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}

fn first_word_v1(s: &String) -> &str {
    let bytes = s.as_bytes();
    // enumerate returns a tuple so we can destructure the tuple
    // enumerates returns a refernce so &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}

fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();
    // enumerate returns a tuple so we can destructure the tuple
    // enumerates returns a refernce so &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    return &s[..];
}
