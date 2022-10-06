use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// This is a limited set of available threads
// we can access and run functions with
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// This represents the functions we can send to the thread to execute
// This type is derived from `std::thread::spawn` params
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // We create a message channel to handle transmitting jobs to threads
        let (sender, receiver) = mpsc::channel();

        // We wrap the receiver in a Mutex to ensure only 1 thread (or Worker) gets the message we send
        let receiver = Arc::new(Mutex::new(receiver));

        // Create an empty Vec for the threads/Workers
        let mut workers = Vec::with_capacity(size);

        // Loop over the number of threads provided
        // and create new Workers and push into Vec
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        // Initialize struct
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a function/closure in an available thread
    ///
    /// # Panics
    ///
    /// The message transmitter will panic if message can't be sent
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // We create a Box to store our function on the stack
        let job = Box::new(f);

        // Then we send it to the first available thread that receives it
        // Since all threads share the same receiver, it goes to all of them
        // (but doesn't run job multiple times due to Mutex below)
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Cleanup method when app exits that closes threads
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Stop receiving messages
        drop(self.sender.take());

        // Loop through each worker and shut down
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

/// These represent threads in our "pool" or queue
/// Each contains a unique ID and an open thread
/// We use message channel to send a "job" to the open thread
///
/// # Panics
///
/// The receiver will panic if it can't lock - this might mean another thread has panicked.
/// The receiver will panic if it can't receive the message - this might mean the thread has shut down.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // We use Arc here to create a thread-safe pointer
    // We use Mutex here to ensure only one Worker at a time requests a job (so we can lock it to receive a message)
    // Our Job is technically a Receiver from the message channel
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("The receiver can't lock - this might mean another thread has panicked.")
                .recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    // Run the function or "job" we originally `execute()`
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
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
