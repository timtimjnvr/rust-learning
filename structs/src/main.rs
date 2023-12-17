struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("thisismyname"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };

    // we can change a field of a mutable struct using dot notation
    user1.email = String::from("newemail@example.com");
    println!(
        "{},{},{},{}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let user2 = build_user(String::from("user2@example.com"), String::from("user2name"));
    println!("{}", user2.username);

    // struct update syntax
    // this :
    let _user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // can be shothanded using .. notation
    let _user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    // Note :
    // user2 is not valid anymore because we used assignement and data was just moved
    // indeed the string username from user1 was moved to user2
    // if we had created a new string for user1 as well, user1 will still be valid
    // other fields : active, sign_in_count are types that implements the Copy trait
    // they are copied on the stack.

    // tuple structs
    // used when naming of the different fields of the tuple would be redondant
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit like structs
    // behave similarly to ()
    struct AlwaysEqual;
    let equals = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// shorthand declaration (for function arguments with same name as struct fields)
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
