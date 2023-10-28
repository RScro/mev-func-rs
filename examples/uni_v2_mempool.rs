
use ethers::prelude::*;
use ethers::abi::AbiDecode;
use std::sync::Arc;
use mev::abi::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Ws>::connect(std::env::var("WSS_URL").expect("missing WSS_URL")).await?;
    let client = Arc::new(&provider);
    println!("[STARTING]");

    let stream = client.subscribe_pending_txs().await?;
    let mut tx_stream = stream.transactions_unordered(usize::MAX);
    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("[TX] {:?}\n[DECODED]{:?}\n", tx.hash, decoded);
            }
        }
    }
    
    Ok(())
}
  