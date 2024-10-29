use std::error::Error;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use eyre::Result;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // Create the provider.
    let rpc_url = "wss://mainnet.infura.io/ws/v3/225ws6xKOwSFZY3t3hM6F1BiIHb";
    let ws = WsConnect::new(rpc_url);
    let provider = ProviderBuilder::new().on_ws(ws).await?;

    // Subscribe to mempool
    let sub = provider.subscribe_pending_transactions().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream();

    println!("Listening for pending transactions...");
    // Take the stream and print the pending transaction.
    let handle = tokio::spawn(async move {
        while let Some(tx_hash) = stream.next().await {
            // Get the transaction details.
            if let Ok(tx) = provider.get_transaction_by_hash(tx_hash).await {
                println!("Transaction details: {tx:#?}");
            }
        }
    });

    handle.await?;
    Ok(())
}