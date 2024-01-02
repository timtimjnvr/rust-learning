// bring trait and type into scope
use generics::{Summary, Tweet};

// generics over struct
struct Point<T> {
    x: T,
    y: T,
}

struct Point_with_two_types<T, U> {
    x: T,
    y: U,
}

// generics over enum
enum Option<T> {
    Some(T),
    None,
}

// genrics over method implementations
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// define a method for only certain instance of points
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point_with_two_different_types<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point_with_two_different_types<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: Point_with_two_different_types<X2, Y2>,
    ) -> Point_with_two_different_types<X1, Y2> {
        Point_with_two_different_types {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };

    // wont compile because x & y are not the same type
    // let wont_work = Point { x: 5, y: 4.0 };

    let _integer_and_float = Point_with_two_types { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = Point_with_two_different_types { x: 5, y: 10.4 };
    let p2 = Point_with_two_different_types { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn _largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn _largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generics over functions
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
