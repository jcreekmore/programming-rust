use std::collections::VecDeque;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub type Work = Box<FnMut() + Send>;
type Queue = Arc<Mutex<VecDeque<Work>>>;

enum Message {
    CheckForWork,
    Quit,
}

struct PoolHandle {
    send: mpsc::Sender<Message>,
    join: thread::JoinHandle<()>,
}

pub struct ThreadPool {
    queue: Queue,
    threads: Vec<PoolHandle>,
}

fn create(queue: Queue, recv: mpsc::Receiver<Message>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match recv.recv().unwrap() {
                Message::Quit => {
                    break;
                }
                Message::CheckForWork => {
                    let work = {
                        let mut queue = queue.lock().unwrap();
                        queue.pop_front()
                    };
                    if let Some(mut thunk) = work {
                        thunk();
                    }
                }
            }
        }
    })
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        let queue: Queue = Arc::new(Mutex::new(VecDeque::new()));
        let mut threads = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let (send, recv) = mpsc::channel();
            let join = create(queue.clone(), recv);
            threads.push(PoolHandle {
                send: send,
                join: join,
            });
        }

        ThreadPool {
            queue: queue,
            threads: threads,
        }
    }

    pub fn add_work<T>(&mut self, work: T)
        where T: FnMut() + Send + 'static
    {
        {
            let mut queue = self.queue.lock().unwrap();
            queue.push_back(Box::new(work));
        }

        for thread in &self.threads {
            thread.send.send(Message::CheckForWork).unwrap();
        }
    }

    pub fn quit(&mut self) {
        for thread in &self.threads {
            thread.send.send(Message::Quit).unwrap();
        }
        for thread in self.threads.drain(..) {
            thread.join.join().unwrap();
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Dropping...");
        self.quit();
        println!("All threads stopped.");
    }
}

fn main() {
    let num_threads = 5;
    let mut pool = ThreadPool::new(num_threads);

    for x in 0..num_threads {
        pool.add_work(move || {
            println!("Work #{}", x);
        });
    }
}
