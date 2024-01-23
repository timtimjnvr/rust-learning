use std::ops::Add;

// this trait can be implemented for any type T

struct Item;

// we will need to use a type parameter to call next
/*
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
 */

// There can be only one implementatoin of Iterator per struct
/* */
pub trait Iterator {
    // this type definition is an Associated types (associated to the trait)
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

// this restricts the type to u32, choosen in this implementation
// we don't need to specify a type parameter when calling next
struct Counter;
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--

        Some(0)
    }
}

// <PlaceholderType=ConcreteType> is a default generic type parameter
// this is the Add implementation
/*
trait Add<Rhs = Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

// implementation for a given type
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// two traits have the fly method
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

// implementation of the two traits
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// definition of a method on human
// if we call fly on human, Rust will call the method directly implemented on the type
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// fmt::Display is a super trait of OutlinePrint and the type that want to implement OutlinePrint need to implement fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // to_string is automatically implemented on the type implementing fmt::Display
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up!
    person.fly(); // *waving arms furiously* (default method)

    // fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // newtype pattern to implement an external trait on a type external to ou crate (here Display on Vec<T>)
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
