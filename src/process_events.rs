use serde_json::{Value, Deserializer};
use log::info;

async fn process_events(receiver: crossbeam_channel::Receiver<Value>, 
handles: Vec<Runtime>) {
    loop {
        match receiver.recv() {
            Ok(event) => {
                info!("Received transaction event: {}", 
serde_json::to_string(&event).unwrap());

                // Process the transaction event by creating a new task 
for each step in the pipeline.
                handles[0].spawn(async move {
                    process_step_1(event).await;
                });
                handles[1].spawn(async move {
                    process_step_2(event).await;
                });
            },
            Err(_) => {
                // If the receiver is disconnected, shut down all worker 
threads and exit.
                info!("Receiver disconnected, shutting down worker 
threads.");

                for handle in handles {
                    handle.shutdown_await();
                }

                std::process::exit(0);
            },
        }
    }
}