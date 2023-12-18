enum FirstIpAddrKind {
    V4,
    V6,
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    // no data associated to it.
    Quit,
    // named fields associated.
    Move { x: i32, y: i32 },
    // String argument
    Write(String),
    // 3 i32 arguments
    ChangeColor(i32, i32, i32),
}

// define methods on enums
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // Create instance of each of the two variants
    // the variants are namespaced under its identifier
    let four = FirstIpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option::Some can hold any type
    let some_number = Some(5); // Option<i32>, rust infers the type
    let some_char = Some('e'); // Option<char>, rust infers the type

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; will panic you need to verify the Option is not a None variant using match !
}

fn route(ip_kind: IpAddrKind) {}
