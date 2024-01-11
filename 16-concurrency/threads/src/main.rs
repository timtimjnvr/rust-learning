use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // starting a new thread
    // move gives ownership of the vector to the new thread
    let t = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        println!("Here's a vector: {:?}", v);
    });

    // this won't compile
    // println!("Here's a vector from the main thread: {:?}", v);

    // waiting for thread to end
    t.join().unwrap();
}
