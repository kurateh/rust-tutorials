use std::{
    error::Error,
    fmt::Display,
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    _workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                // `receiver.lock()`에서 임시로 얻은 `MutexGuard`는 job을 얻는 순간 버려짐 => unlock
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing.");

                job();
            }
        });
        Worker {
            _id: id,
            _thread: thread,
        }
    }
}

#[derive(Debug)]
struct PoolCreationError {
    message: String,
}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for PoolCreationError {}

impl ThreadPool {
    /// Create a new `TreadPool`.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `build` function will panic if the size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, Box<dyn Error>> {
        if size <= 0 {
            return Err(Box::new(PoolCreationError {
                message: String::from("Size must be greater than 0"),
            }));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            _workers: workers,
            sender,
        })
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}
