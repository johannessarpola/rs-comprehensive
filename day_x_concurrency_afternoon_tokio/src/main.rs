
use std::thread;

use tokio::time;

use tokio::sync::mpsc::{self, Receiver};

async fn ping_handler(mut input: Receiver<()>) {
    let mut count: usize = 0;

    while let Some(_) = input.recv().await {
        count += 1;
        println!("Received {count} pings so far in {} with id {:?}!", thread::current().name().unwrap(), thread::current().id());
    }

    println!("ping_handler complete");
}

async fn count_to(from: i32, to: i32) {
    for i in from..=to {
        println!("Count in task: {i} in {} with id {:?}!", thread::current().name().unwrap(), thread::current().id());
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(0, 100)); // this gets canceled as it is not awaited as main finishes
    let h2  = tokio::spawn(count_to(201, 221)); // this gets canceled as it is not awaited as main finishes

    count_to(15, 20).await; // runs on main thread

    let _ = tokio::spawn(count_to(99, 101)).await; // runs o worker thread
    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await; // main
    }

    let _ = h2.await; // waits for handle to finish

    let (sender, receiver) = mpsc::channel(32);
    let ping_handler_task = tokio::spawn(ping_handler(receiver));
    for i in 0..10 {
        sender.send(()).await.expect("Failed to send ping.");
        println!("Sent {} pings so far.", i + 1);
        time::sleep(time::Duration::from_millis(5)).await;
    }

    drop(sender);
    ping_handler_task.await.expect("Something went wrong in ping handler task.");
}
