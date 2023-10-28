#![allow(unused)]

use ethers::prelude::*;
use std::sync::Arc;
use mev::{abi::*,functions::*};


#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Ws>::connect(std::env::var("WSS_URL").expect("missing WSS_URL")).await?;
    let client = Arc::new(&provider);
    println!("[STARTING]");

    let sub_blocks = subscribe_blocks(client.clone());
    let mempool = monitor_mempool(client);
    tokio::join!(sub_blocks, mempool);

    Ok(())
}