use std::sync::mpsc;
use std::{thread, time};

fn main() {
    // mpsc = multiple producer, single consumer
    // tx is transmitter and rx is reiceiver
    let (tx, rx) = mpsc::channel();

    // non blocking rcv
    match rx.try_recv() {
        Ok(received) => println!("Got: {}", received),
        Err(error) => println!("Error : {}", error),
    }

    thread::spawn(move || {
        thread::sleep(time::Duration::from_millis(1000));
        let val1 = String::from("first");
        let val2 = String::from("second");
        tx.send(val1).unwrap();
        tx.send(val2).unwrap();
    });

    // blocking until a value is sent down the channel.
    let mut received = rx.recv().unwrap();
    println!("Got: {}", received);

    received = rx.recv().unwrap();
    println!("Got: {}", received);

    match rx.recv() {
        Ok(received) => println!("Got: {}", received),
        Err(error) => println!("Error : {}", error),
    };

    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx2.send(val).unwrap();
        // this won't compile as ownership has been transferred to receiver
        // println!("val is {}", val);
    });

    let received = rx2.recv().unwrap();
    println!("Got: {}", received);

    let (tx3, rx3) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx3.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });
    // this is a blocking receive
    for received in rx3 {
        println!("Got: {}", received);
    }

    // multiple senders, one receiver
    let (tx4, rx4) = mpsc::channel();
    let tx5 = tx4.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx5.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx4.send(val).unwrap();
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    for received in rx4 {
        println!("Got: {}", received);
    }
}
