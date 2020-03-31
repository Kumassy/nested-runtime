use tokio;
use tokio::runtime::{Runtime, Builder};
use std::time::Duration;
use tokio::task;

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let runtime = Runtime::new()?;
    let handle = runtime.handle();
    handle.spawn(async move {
        // do stuff
        println!("from handle");
    });

    task::spawn_blocking(move || {
       runtime.shutdown_timeout(Duration::from_millis(2000));
    });
    Ok(())
}