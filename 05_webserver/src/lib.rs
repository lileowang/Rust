#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

/// Author:     Li Leo Wang
/// Date:       2020-06-28
/// Description:
/// - Based on Rust the Book.
/// - Not working in Chrome.
/// - Works in Firefox.
/// 

use std::fmt::{Debug, Display};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}

// refer to thread::spawn() declaration
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Message>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            log(&format!("worker {}: waiting for job", id));
            let message = rx.lock().unwrap().recv().unwrap();
            log(&format!("worker {}: received job", id));
            match message {
                Message::NewJob(job) => {
                    log(&format!("worker {}: executing job", id));
                    job();
                    log(&format!("worker {}: finished job", id));
                }
                Message::Terminate => {
                    log(&format!("worker {}: terminating", id));
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool { workers, tx }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.tx.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        log(&format!("terminate message: to all workers"));
        for _ in &self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }

        log(&format!("terminate message: all out"));
        for worker in &mut self.workers {
            log(&format!("worker {}: wait to shut down", worker.id));
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            log(&format!("worker {}: finished shutting down", worker.id));
        }
    }
}
