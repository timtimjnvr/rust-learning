use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex is a wrapper around the value it holds
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();

        // mutex is locked
        println!("m = {:?}", m);

        *num = 6;

        // mutex are automatically released when they go out of scope
    }

    // mutex is unlocked
    println!("m = {:?}", m);

    // MUTEX IN MULTI THREADED SCENARIO

    // we can't give the mutex multiple ownership with Rc because Rc is not thread safe
    // we need to use Arc which has the same API as Rc but is usable in multi threaded scenario
    // counter is immutable !
    // Mutex provides interior mutability !
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
