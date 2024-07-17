# solana-fee

>In this advanced version of the code, we are building a producer-consumer architecture using Rust with async/await to process Solana transactions in parallel. Here's an explanation of each part:

1. `Cargo.toml` is our project's configuration file, which includes 
dependencies such as `tokio`, `crossbeam-channel`, and logging libraries 
like `env_logger` and `log`.
2. `config.rs` sets up the main function to read command line arguments 
and environment variables for Solana URL and database URL. It initializes 
the logger using `env_logger` and starts multiple producer threads along 
with one consumer thread using 
`tokio::runtime::Builder::new_multi_thread()`.
3. The `process_transactions.rs` module is responsible for connecting to 
the Solana node, listening for new transactions, deserializing them, and 
sending them to the event loop for further processing using a 
`crossbeam_channel::Sender<Value>`. It also handles error cases such as 
connection errors and deserialization errors.
4. The `process_events.rs` module is responsible for receiving transaction 
events from the producer threads and passing them on for further 
processing in multiple worker threads. If the receiver is disconnected, it 
shuts down all worker threads and exits the program.
5. `Error.rs` defines custom error types for connection errors and 
deserialization errors that can be used with `snafu`.
6. The main entry point `main.rs` sets up the Solana node URL, database 
URL, and starts the entire system by calling the `process_events()` 
function.



## Installation

Clone this repository and navigate to the project directory:
```bash
$ git clone https://github.com/Aditya-dom/solana_fee.git
$ cd solana_fee
```

Next, add `[dependencies]` to your `Cargo.toml` file if you don't already 
have it:

```toml
[dependencies]
tokio = { version = "1.20.1", features = ["full"] }
crossbeam-channel = "0.5.3"
serde_json = "1.0.64"
env_logger = "0.9.0"
log = "0.4.11"
snafu = { version = "0.7.2", features = ["derive"] }
```

Run `cargo build --release` to build the project in release mode.

## Usage

To start the program, run:

```bash
$ cargo run -- --solana_url="http://localhost:8000" 
--database_url="postgresql://user:password@localhost/dbname?sslmode=disable--database_url="postgreql://user:password@localhost/dbname?sslmode=disable"
```

Replace `--solana_url` and `--database_url` with the desired Solana node 
URL and database connection string.

## Building from Source

You can also build the project from source using Cargo:

```bash
$ cargo build --release
```

The built binary will be located in the `target/release` directory as 
`solana_transaction_processor`.

