use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A job for a worker to complete.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A message source for workers.
type WorkerSender = mpsc::Sender<Message>;

/// A message recipient for workers.
type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Message>>>;

/// A worker's thread.
type WorkerThread = Option<thread::JoinHandle<()>>;

/// A managed pool of workers.
pub struct ThreadPool {
    /// The list of workers to assign work to.
    workers: Vec<Worker>,
    /// The object that sends messages to workers.
    sender: WorkerSender,
}

impl ThreadPool {
    /// Returns a new thread pool of the given size.
    ///
    /// ### Arguments
    ///
    /// * `size` - The number of workers to allocate.
    ///
    /// ### Example
    ///
    /// ```
    /// mod pool;
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = ThreadPool::get_messengers();
        let workers = ThreadPool::get_workers(size, receiver);
        ThreadPool { workers, sender }
    }

    /// Executes the given job asynchronously.
    ///
    /// ### Arguments
    ///
    /// * `f` - The job to complete.
    ///
    /// ### Example
    ///
    /// ```
    /// mod pool;
    /// let pool = ThreadPool::new(4);
    /// pool.execute(|| println!("Doing work!"));
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("Could not send job to workers.");
    }

    /// Returns a sender and receiver to share among pool workers.
    fn get_messengers() -> (WorkerSender, WorkerReceiver) {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        (sender, receiver)
    }

    /// Returns a list of workers that get messages from the given receiver.
    ///
    /// ### Arguments
    ///
    /// * `size` - The number of workers to allocate.
    /// * `receiver` - The message recipient object to share among workers.
    fn get_workers(size: usize, receiver: WorkerReceiver) -> Vec<Worker> {
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        workers
    }
}

impl Drop for ThreadPool {
    /// Terminates all workers for a graceful shutdown.
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Could not tell workers to terminate.");
        }
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect(format!("Could not terminate worker {}", worker.id).as_str());
            }
        }
    }
}

/// A message to be received by workers.
enum Message {
    NewJob(Job),
    Terminate,
}

/// A worker that performs asynchronous work.
struct Worker {
    /// The ID of this worker.
    id: usize,
    /// The thread to execute work on.
    thread: WorkerThread,
}

impl Worker {
    /// Returns a new worker with the given ID and receiver.
    ///
    /// ### Arguments
    ///
    /// * `id` - The ID to assign to this worker.
    /// * `receiver` - The message recipient object for this worker to listen on.
    fn new(id: usize, receiver: WorkerReceiver) -> Worker {
        let thread = thread::spawn(move || loop {
            match Worker::get_message(&receiver) {
                Message::NewJob(job) => {
                    job();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    /// Blocks execution and returns a message when it's received.
    ///
    /// ### Arguments
    ///
    /// * `receiver` - The message recipient object to get messages from.
    fn get_message(receiver: &WorkerReceiver) -> Message {
        receiver
            .lock()
            .expect("Could not lock receiver.")
            .recv()
            .expect("Could not receive message.")
    }
}
