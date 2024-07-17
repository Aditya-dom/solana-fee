use std::env;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    solana_url: String,
    database_url: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [SOLANA_URL] [DATABASE_URL]", args[0]);
        std::process::exit(1);
    }

    let config: Config = serde_json::from_str(&format!("{{{}}}", 
env!("APP_CONFIG"))).unwrap();
    env_logger::init().unwrap();

    let (sender, receiver) = crossbeam_channel::unbounded();

    // Spawn multiple threads to process transactions in parallel.
    for _ in 0..10 {
        tokio::spawn(async move {
            process_transactions(config.solana_url, config.database_url, 
sender).await;
        });
    }

    let mut handles = Vec::<tokio::runtime::Runtime>::new();
    for handle in tokio::runtime::Builder::new_multi_thread().build_all() 
{
        handles.push(handle);
    }

    // Run the main event loop, which receives transactions from the 
producer threads and processes them.
    process_events(receiver, handles).await;
}
