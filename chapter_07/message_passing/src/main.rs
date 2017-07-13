use std::io::{self, Write};
use std::sync::mpsc;
use std::{thread, time};

fn do_large_read() -> String {
    let naptime = time::Duration::from_secs(10);
    thread::sleep(naptime);
    return "really large file... seriously".into();
}

fn do_background_read() -> mpsc::Receiver<String> {
    let (send, recv) = mpsc::channel();

    let _ = thread::spawn(move || {
        let data = do_large_read();
        send.send(data).unwrap();
    });

    recv
}

fn do_foreground_work() {
    print!(".");
    io::stdout().flush();
    let naptime = time::Duration::from_secs(1);
    thread::sleep(naptime);
}

fn main() {
    let recv = do_background_read();

    loop {
        match recv.try_recv() {
            Ok(data) => {
                println!("{:?}", data);
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("disconnected without data");
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                do_foreground_work();
            }
        };
    }
}
