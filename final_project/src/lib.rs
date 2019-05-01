/**
 * Further development ideas:
 * Add more documentation to ThreadPool and its public methods.
 * Add tests of the library’s functionality.
 * Change calls to unwrap to more robust error handling.
 * Use ThreadPool to perform some task other than serving web requests.
 * Find a thread pool crate on https://crates.io/ and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.
 */

use std::thread;

use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type Job = Box<FnBox + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
    (*self)()
  }
}

struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        let message = receiver.lock().unwrap().recv().unwrap();

        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job, executing.", id);
            job.call_box();
          },
          Message::Terminate => {
            println!("Worker {} was told to terminate", id);
            break;
          },
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
  /// Create a new ThreadPool
  /// 
  /// The size is the number of threads in the pool
  /// 
  /// # Panics
  /// 
  /// The `new` function will panic if the size is zero
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool {
      workers,
      sender,
    }
  }

  // TODO: write body with the following signature
  // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    // We’re iterating over the workers twice: once to send one Terminate 
    // message for each worker and once to call join on each worker’s thread. 
    // If we tried to send a message and join immediately in the same loop, we 
    // couldn’t guarantee that the worker in the current iteration would be the 
    // one to get the message from the channel.

    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);

      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}