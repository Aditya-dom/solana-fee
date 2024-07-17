use snafu::{ResultExt, RecoverableError};
use tokio::runtime::Runtime;
use tokio::net::TcpStream;
use serde_json::{Value, Deserializer};
use log::{debug, error, info};

#[derive(RecoverableError)]
enum Error {
    #[snafu(display("Failed to connect to Solana node: {}", source))]
    ConnectionError { source: std::io::Error },
    #[snafu(display("Failed to deserialize Solana message: {}", source))]
    DeserializationError { source: Box<DeserializationError> },
}

async fn process_transactions(solana_url: String, database_url: String, 
sender: crossbeam_channel::Sender<Value>) {
    let mut handle = Runtime::new().unwrap();

    // Connect to the Solana node and listen for new transactions.
    let mut stream = TcpStream::connect(solana_url).await.context("Failed 
to connect to Solana node")?;

    loop {
        let message = stream.read_all(&mut [0; 1024]).await.context("Error 
reading from Solana socket")?;
        let value: Value = 
serde_json::from_slice(&message).context("Failed to deserialize Solana 
message")?;

        // Send the transaction data to the event loop for further 
processing.
        sender.send(value).unwrap();

        handle.block_on(async {
            stream.shutdown(std::net::Shutdown::Both).await;
        }).expect("Failed to shutdown Solana connection");

        // Start a new connection and listen for new transactions.
        let mut stream = 
TcpStream::connect(solana_url).await.context("Failed to connect to Solana 
node")?;
    }
}