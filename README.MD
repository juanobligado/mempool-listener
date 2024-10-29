# Mempool Listener

This project is a Rust-based application that listens to the Ethereum mempool for pending transactions. It uses the `alloy` crate to connect to an Ethereum node via WebSocket and processes transactions in real-time.

## Features

- Connects to an Ethereum node using WebSocket.
- Subscribes to pending transactions in the mempool.
- Filters and processes transactions to specific contract addresses.
- Prints transaction details to the console.

## Dependencies

- `tokio`: For asynchronous programming.
- `alloy`: For Ethereum interaction.
- `eyre`: For error handling.
- `futures_util`: For working with streams.

## Installation

1. Ensure you have Rust and Cargo installed. If not, you can install them from [here](https://www.rust-lang.org/tools/install).

2. Clone the repository:
   ```sh
   git clone https://github.com/juanobligado/mempool-listener.git
   cd mempool-listener