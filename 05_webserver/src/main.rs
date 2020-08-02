#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

/// Author:     Li Leo Wang
/// Date:       2020-06-28
/// Description:
/// - Based on Rust the Book.
/// - Not working in Chrome:
///   when buffer size is 512,
///   works when increased to 1024
/// - Works in Firefox.
/// - Dead lock in Chrome when shutting down!
///
mod lib;

use lib::log;
use lib::ThreadPool;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    log(&"start web server");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let stdin_channel = spawn_stdin_channel();
    let pool = ThreadPool::new(4);

    // Single-threaded server:
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     log(&"connection established.");
    //     handle_connection(stream);
    // }

    // Mult-threaded server:
    //for stream in listener.incoming().take(4) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // log(&"established");
        // handle_connection(stream);
        pool.execute(|| {
            handle_connection(stream)
        });

        let mut key = match stdin_channel.try_recv() {
            Ok(key) => key,
            Err(TryRecvError::Empty) => String::from("no input from console"),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        };

        key = key.trim().to_string();
        if key.eq_ignore_ascii_case("q") {
            println!("quit");
            break;
        } else if key.eq_ignore_ascii_case("cls") {
            println!("clear screen");
            Command::new("cmd")
                .args(&["/C", "cls"])
                .spawn()
                .expect("failed to clear screen");
        } else if key.eq_ignore_ascii_case("no input from console") {
            // println!("{}", key);
        } else {
            println!("Received: {}", key);
        }
    }

    log(&"shutting down server");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // log(&format!("request: {}", String::from_utf8_lossy(&buffer[..])));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "sleep.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    // log(&format!("status_line: {}, filename: {}", &status_line, &filename));
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}
