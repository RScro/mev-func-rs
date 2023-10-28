#![allow(unused)]

use ethers::{prelude::*, utils::{parse_ether, format_units}};
use std::sync::Arc;
use mev::abi::*;

/// Example on how to find an arbitrage opportunity
/// between 2 pairs on different exchanges.
#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::<Ws>::connect(std::env::var("WSS_URL").expect("missing WSS_URL")).await?;
    let client = Arc::new(&provider);
    println!("[STARTING]");

    let uni_factory_address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse::<Address>()?;
    let uni_factory = UniV2Factory::new(uni_factory_address, client.clone());

    let sushi_factory_address = "0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac".parse::<Address>()?;
    let sushi_factory = UniV2Factory::new(sushi_factory_address, client.clone());

    // This is WETH & USDT, replace with any tokens you wish
    let weth_address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;
    let usdt_address = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse::<Address>()?;

    let uni_pair_address = uni_factory
    .get_pair(weth_address, usdt_address)
    .call()
    .await
    .unwrap();

    let sushi_pair_address = sushi_factory
    .get_pair(weth_address, usdt_address)
    .call()
    .await
    .unwrap();

    println!("\nPair Address UNI : {:?}\nPair Address SUSHI : {:?}", uni_pair_address, sushi_pair_address);

    // Now we can also call the get_Reserves function on
    // Both pairs to see if there's an arbitrage opportunity.

    let uni_pair = UniV2Pair::new(uni_pair_address, client.clone());
    let sushi_pair = UniV2Pair::new(sushi_pair_address, client.clone());

    let uni_pair_reserves = uni_pair
    .get_reserves()
    .call()
    .await
    .unwrap();

    let sushi_pair_reserves = sushi_pair
    .get_reserves()
    .call()
    .await
    .unwrap();

    // Uniswap Reserves
    let uni_reserve0 = U256::from(uni_pair_reserves.0);
    let uni_reserve1 = U256::from(uni_pair_reserves.1);

    // Sushiswap Reserves
    let sushi_reserve0 = U256::from(sushi_pair_reserves.0);
    let sushi_reserve1 = U256::from(sushi_pair_reserves.1);

    // After we get the reserves, we call 
    // the get_Amount_Out function on both routers.

    let uni_router_address = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    let uni_router = UniV2Router::new(uni_router_address, client.clone());

    let sushi_router_address = "0xd9e1cE17f2641f24aE83637ab66a2cca9C378B9F".parse::<Address>()?;
    let sushi_router = UniV2Router::new(sushi_router_address, client.clone());

    // Represents 1 ETH.
    let eth1 = parse_ether(1).unwrap();

    let uni_price = uni_router
    .get_amount_out(eth1, uni_reserve0, uni_reserve1)
    .call()
    .await
    .unwrap();

    let sushi_price = sushi_router
    .get_amount_out(eth1, sushi_reserve0, sushi_reserve1)
    .call()
    .await
    .unwrap();

    println!("\nUni Pair Price : {:?}\nSushi Pair Price : {:?}", 
    format_units(uni_price, 5).unwrap(), 
    format_units(sushi_price, 5).unwrap());

    Ok(())
}
