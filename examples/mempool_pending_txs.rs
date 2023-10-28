
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Ws>::connect(std::env::var("WSS_URL").expect("missing WSS_URL")).await?;
    let client = Arc::new(&provider);
    println!("[STARTING]");

    let stream = client.subscribe_pending_txs().await?;
    let mut tx_stream = stream.transactions_unordered(usize::MAX);
    while let Some(tx) = tx_stream.next().await {
        if let Ok(tx) = tx {
            println!("[TX] {:?}", tx.hash);
        }
    }
    
    Ok(())
}
