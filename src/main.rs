use tokio;
use tokio::runtime::{Runtime, Builder};
use std::time::Duration;

#[tokio::main(core_threads = 1)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let runtime = Runtime::new()?;
    let handle = runtime.handle();
    handle.spawn(async move {
        // do stuff
        println!("from handle");
    });

    runtime.shutdown_timeout(Duration::from_millis(1000));
    Ok(())
}