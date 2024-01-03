// holds a reference to a String slice owned by some variable created before struct instanciation.
// Rust will force that the variable doesn’t go out of scope until after the ImportantExcerpt goes out of scope
// the reference in the ImportantExcerpt instance will be valid.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }
    // wont compile because x reference is not valid anymore
    // println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    // takes string slice as arguments (which are references)
    // because we don't want the longest function to take ownership of the strings
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    {
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
}

// the lifetime notation below means : the returned reference will be valid as long as both the parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;
// generic parameters types definition and lifetime annotations goes in the same place (before function parameters).
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
