#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::fmt::{Debug, Display};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn log<T>(msg: &T)
where
    T: Display + Debug,
{
    println!("{}", msg);
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    tool: Option<thread::JoinHandle<()>>,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Message>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<Receiver<Message>>>) -> Self {
        let tool = thread::spawn(move || loop {
            log(&format!("worker {}: wait for job", id));
            let msg = rx.lock().unwrap().recv().unwrap();
            log(&format!("worker {}: got the job", id));
            match msg {
                Message::NewJob(n) => {
                    log(&format!("worker {}: executing the job", id));
                    n();
                    log(&format!("worker {}: finished the job", id));
                }
                Message::Terminate => {
                    log(&format!("worker {}: terminating", id));
                    break;
                }
            }
        });

        Worker {
            id,
            tool: Some(tool),
        }
    }
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let mut workers = Vec::<Worker>::with_capacity(size);
        let (tx, rx) = mpsc::channel::<Message>();
        let rx = Arc::new(Mutex::new(rx));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }
        ThreadPool {
            workers,
            tx,
        }
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
        log(&"terminate message: to all workers");
        for _ in &self.workers {
            self.tx.send(Message::Terminate).unwrap();
        }

        log(&"termiante message: all out");
        for worker in &mut self.workers {
            log(&format!("worker {}: wait to shut down", worker.id));
            if let Some(tool) = worker.tool.take() {
                tool.join().unwrap();
            }
            log(&format!("worker {}: finished shutting down", worker.id));
        }
    }
}
