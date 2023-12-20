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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // this match all other possible cases !
        // need to be put at last because patterns are evaluated in order.
        other => move_player(other),
        // this pattern evaluates to all other types but does not bind the value.
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}

fn route(ip_kind: IpAddrKind) {}

fn value_in_cents(coin: Coin) -> u8 {
    // arms MUST cover all possibilities ! prevents from unexpected scenarios.
    // if a match is missing a variant, it won't compile.
    match coin {
        // evaluates in order
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
