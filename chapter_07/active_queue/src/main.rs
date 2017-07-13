use std::sync::mpsc;
use std::thread;

pub type Thunk = Box<FnMut() + Send>;

pub struct ActiveQueue {
    send: mpsc::SyncSender<Thunk>,
    join: thread::JoinHandle<()>,
}

impl ActiveQueue {
    pub fn new(bound: usize) -> ActiveQueue {
        let (send, recv) = mpsc::sync_channel(bound);

        let join = thread::spawn(move || {
            loop {
                let mut thunk: Thunk = recv.recv().unwrap();
                thunk();
            }
        });

        ActiveQueue {
            send: send,
            join: join,
        }
    }

    pub fn send<T: FnMut() + Send + 'static>(&self, thunk: T) {
        self.send.send(Box::new(thunk)).unwrap();
    }

    pub fn wait(self) {
        self.join.join().unwrap();
    }
}

fn main() {
    let queue = ActiveQueue::new(10);

    queue.send(|| {
        println!("Hello, world!");
    });

    queue.send(|| {
        println!("Hello, world again!");
    });

    queue.wait();
}
