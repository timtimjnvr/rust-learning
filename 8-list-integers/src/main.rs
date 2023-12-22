mod integers_vector;

fn main() {
    let mut v = integers_vector::new_list();

    let list = [1, 20, 3, 3, 42, 5];
    for e in list {
        v.add(e);
    }

    match &v.median() {
        None => println!("Empty array"),
        Some(i) => println!("median is {}", i),
    }

    match &v.mode() {
        None => println!("Empty array"),
        Some(i) => println!("mode is {}", i),
    }

    let new_v = integers_vector::new_list();
    match new_v.median() {
        None => println!("Empty array"),
        Some(i) => println!("median is {}", i),
    }

    match &new_v.mode() {
        None => println!("Empty array"),
        Some(i) => println!("mode is {}", i),
    }
}
