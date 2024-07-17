#[tokio::main]
async fn main() -> Result<(), Error> {
    let solana_url = "http://localhost:8899"; // Update this to your desired 
Solana RPC endpoint
    let client = RpcClient::new(solana_url);

    let (tx, rx) = mpsc::channel(16);
    let processor = Arc::new(|transaction: SolanaTransaction| 
process_events(transaction));

    tokio::spawn(async move {
        process_transactions(client, tx, processor).await?;
    });

    loop {
        match rx.recv().await {
            Ok(_) => (), // Process the transaction and continue with the next 
one.
            Err(e) => {
                println!("Error receiving message from transaction processing: 
{}", e);
                break;
            },
        }
    }

    Ok(())
}
