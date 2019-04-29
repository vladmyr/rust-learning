use std::thread;

pub struct ThreadPool {
  threads: Vec<thread::JoinHandle<()>>,
};

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

    let mut threads = Vec::with_capasity(size);

    for _ in 0..size {
      // create some threads and sotre them in the vectior
    }

    ThreadPool
  }

  // TODO: write body with the following signature
  // pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError>

  pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static
  {
    
  }
}