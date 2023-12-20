#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// all methods inside will be associated functions
impl Rectangle {
    // define a method on the Rectangle type
    // &self instead of rectangle: &Rectangle
    // &self = &Self
    // type Self -> alias for the type that the impl block is for.
    // & in '&self' to indicate that this method borrows the Self instance (we want to only read data)
    // if we want to modify the data in the method we can use &mut self

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods can have the same name as fields
    fn width(&self) -> bool {
        self.width > 0
    }

    // takes an immutable reference to another Rectangle because we don't want ownership of the data
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated function that is not a method of Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // with variables
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_v0(width1, height1)
    );

    // with tuple
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_v1(rect1)
    );

    // with structs
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // area_v2 takes a reference of rectangle rather than the struct because we only need to borrow it, not to take ownership of the data.
        area_v2(&rect1)
    );

    // traits

    // :? in println because we want println to use Debug as an output
    println!("rect1 is {:?}", rect1);

    // :#? allows to have a nicer print (userful for bigger structs)
    println!("rect1 is {:#?}", rect1);

    // dbg! takes ownership of an expression, prints it to stderr with line number and then returns ownership
    // as opposed to println!, which takes a reference
    // to avoid dbg to take ownership, we can use reference
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("{}", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
}

fn area_v0(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
