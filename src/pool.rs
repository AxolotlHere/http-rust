use std::io;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(n);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..n {
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }

        ThreadPool {
            workers: workers,
            _sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self._sender.send(job).unwrap();
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    _sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();
                job();
            }
        });
        Worker {
            id: id,
            _thread: thread::spawn(move || ()),
        }
    }
}
