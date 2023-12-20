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
}
