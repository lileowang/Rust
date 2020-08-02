#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod lib;

use lib::log;
use lib::ThreadPool;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::stdin;
use std::net::{TcpListener, TcpStream};
use std::process;
use std::process::Command;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    log(&"start server");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let rx = spawn_stdin_channel().unwrap_or_else(|err| {
        log(&err);
        process::exit(1);
    });
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });

        let input = match rx.try_recv() {
            Ok(n) => n,
            Err(TryRecvError::Empty) => String::from("empty"),
            Err(TryRecvError::Disconnected) => {
                log(&"disconnected from stdin");
                process::exit(2);
            }
        };

        if input.eq_ignore_ascii_case("q") {
            log(&"quit");
            break;
        } else if input.eq_ignore_ascii_case("cls") {
            Command::new("cmd")
                .args(&["/c", "cls"])
                .spawn()
                .expect("failed to clear screen");
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    // log(&String::from_utf8_lossy(&buffer[..]));

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

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn spawn_stdin_channel() -> Result<Receiver<String>, Box<dyn Error>> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("failed to read input");
        tx.send(buffer.trim().to_string()).unwrap();
    });
    Ok(rx)
}
