use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    println!("Thread Examples (Ch16)");
    //threadspawn();
    //threadjoinmiddle();
    //threadjoinafter();
    //threadbadclosure();
    channeltxrx();
}

fn threadspawn() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn threadjoinmiddle() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn threadjoinafter() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn threadbadclosure() {
    let v = vec![1, 2, 3];

    // Problem!  Lifetime needs a move.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // Can't do this after because it got moved.
    //println!("Here's a vector: {:?}", v);
    handle.join().unwrap();
} 

fn channeltxrx() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

