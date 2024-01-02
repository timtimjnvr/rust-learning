use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    // just signature
    // fn summarize(&self) -> String;

    // default implementation
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// for is followed by the name of the type on which we want to implement the trait
//  we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct SomeType {}

// use default trait implementation
impl Summary for SomeType {}

// trait as parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait as parameter using trait bound syntax
pub fn _notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// specifying more than on trait in parameter
pub fn notify_and_display(_item: &(impl Summary + Display)) {}

// usage of where to define trait parameter definition
// with trait bound
fn _some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// return a trait (however one function can only return one type)
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

// this implementation applies for any type
impl<T> Pair<T> {
    fn _new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// implement methods conditionnaly on types that implements certain rules
impl<T: Display + PartialOrd> Pair<T> {
    fn _cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
// blanket implementation : implement a method for type (T) that implements a trait (here Display)
// impl<T: Display> ToString for T {
//     --snip--
// }
