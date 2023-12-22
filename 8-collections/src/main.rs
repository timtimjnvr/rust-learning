// need to be imported because it is not included in the prelude containing the std library
use std::collections::HashMap;

fn main() {
    // Vectors

    // Rust can't infer type of an empty vector
    let v: Vec<i32> = Vec::new();

    // defining a vector using vec !
    let v = vec![1, 2, 3];

    // defining a vector and pushing data into it.
    // No need to specify type because Rust infers the type i32 because of the push calls.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    // accessing data in a vector thanks to indexes
    // using & returns a reference to the element
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // accessing data with get
    // allow to handle panic in case of vector access beyond size
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // when retrieving an element of the vector the borrow checker enforces the ownership and borrowing rules
    // to ensure this reference and any other references to the contents of the vector remain valid.
    // Reminder : you can't have mutable and immutable reference in the same scope.

    // iterating on a vector
    let v = vec![100, 32, 57];

    // iterate and get immutable reference
    for i in &v {
        println!("{i}");
    }

    // iterate and get mutable reference (to make changes)
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // String

    // create from scratch
    let mut s = String::new();

    // create from data
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    // with from
    let s = String::from("initial contents");

    // String Update

    let mut s = String::from("foo");
    // append
    // takes a string slice in parameter because we don't want to take ownership of the parameter.
    s.push_str("bar");

    // here we want to be able to use s2 after appending it to s1
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // if push_str took ownership of s2 we would not be able to print s2
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    // push append a single character to the string
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // adding a reference of s2 ot s1
    // actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.
    let s3 = s1 + &s2;
    // note s1 has been moved and can no longer be used
    // we use a reference on the 2nd string because of the signature of the method called behind the sceene
    // fn add(self, s: &str) -> String
    // the compiler will coerce the &String argument into a &str (&s2 into &s2[..])

    let new_s1 = String::from("tic");
    let new_s12 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = new_s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{new_s12}-{s2}-{s3}");

    for c in "Зд".chars() {
        println!("{c}");
    }
    // З
    // д

    for b in "Зд".bytes() {
        println!("{b}");
    }
    // 208
    // 151
    // 208
    // 180

    // Hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // if there's no value associated to "Blue", get will return None (get returns an Option<&V>).
    // the function copied handles the option to get Option<i32> rather than an Option<&i32>
    // unwrap or zero set score to 0 if there's no entry for "Blue"
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // iteration over a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // to insert a key only if it does not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    // split_whitespace returns an iterator
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
