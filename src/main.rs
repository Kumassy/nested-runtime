use tokio::runtime::{Runtime, Builder};
use tokio::task;
use tokio::sync::mpsc;
use tokio::time;

use std::fs::File;
use std::time::{Duration, Instant};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

use pretty_env_logger;
use log::{error, warn, info, debug};

struct Spy {
    id: String
}
impl Spy {
    fn new(id: String) -> Self {
        Spy {id: id}
    }

    fn peek(&self) {}
}
impl Drop for Spy {
    fn drop(&mut self) {
        println!("dropping {}...", self.id);
    }
}

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    println!("Hello, world!");

    let runtime = Runtime::new()?;
    let handle = runtime.handle();
    let (mut tx, mut rx) = mpsc::unbounded_channel();

    let s1 = Spy::new("receiver-thread".into());
    handle.spawn(async move {
        while let Some(_) = rx.recv().await {
            let mut buf = vec![0u8; 4096];
            let mut reader_file = BufReader::new(File::open("Cargo.toml").unwrap());
            let len = match task::block_in_place(|| reader_file.read(&mut buf)) {
                Ok(n) if n == 0 => return,
                Ok(n) => n,
                Err(e) => {
                    warn!("faild to read from tap; err = {:?}", e);
                    return;
                }
            };
            println!("{:?}", &buf[..len]);
            s1.peek();
        }
    });

    let s2 = Spy::new("thread-send".into());
    handle.spawn(async move {
        loop {
            time::delay_for(Duration::from_millis(1000)).await;
            tx.send(()).unwrap();

            s2.peek();
        }
    });

    let s3 = Spy::new("outside-of-thread".into());
    s3.peek();

    task::spawn_blocking(move || {
       runtime.shutdown_timeout(Duration::from_millis(2000));
        println!("shutdown");
    });

    Ok(())
}