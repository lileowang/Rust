#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt::{Debug, Display};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    let tx1 = Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![String::from("a1"), String::from("a2"), String::from("a3")];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("b1"), String::from("b2"), String::from("b3")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r in rx {
        log(&format!("got: {}", r));
    }
}

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}
