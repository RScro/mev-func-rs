#![allow(unused)]

use ethers::prelude::*;
use std::sync::Arc;
use ethers::abi::AbiDecode;
use crate::abi::*;

/// Subscribe to new pending transactions in the mempool.
pub async fn monitor_mempool(client: Arc<&Provider<Ws>>) {
    let stream = client.subscribe_pending_txs().await.unwrap();
    let mut tx_stream = stream.transactions_unordered(usize::MAX);
    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("[TX] {:#?}\n[DECODED]{:#?}\n", tx.hash, decoded);
            }
        }
    }
}

/// Subscribe to new blocks.
pub async fn subscribe_blocks(client: Arc<&Provider<Ws>>) {
    let mut stream = client.subscribe_blocks().await.unwrap();
    while let Some(block) = stream.next().await {
       println!("[BLOCK NUMBER] - {:?}", block.number.unwrap_or_default());
    }
}