
use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Ws>::connect(std::env::var("WSS_URL").expect("missing WSS_URL")).await?;
    let client = Arc::new(&provider);
    println!("[STARTING]");

    let mut stream = client.subscribe_blocks().await?;
    while let Some(block) = stream.next().await {
       println!("[BLOCK NUMBER] - {:?}", block.number.unwrap_or_default());
    }
    Ok(())
}
