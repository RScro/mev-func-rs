
use ethers::prelude::*;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenv::dotenv().ok();
    let provider = Provider::try_from(std::env::var("RPC_URL").expect("missing RPC_URL"))?;
    let chain_id = provider.get_chainid().await?;
    let block_number = provider.get_block_number().await?;
    println!("\nChain id: {:?}, Block number: {:?}\n", chain_id, block_number);

    // Private key has to be the private key from account1
    let private_key: LocalWallet = std::env::var("PRIVATE_KEY").expect("missing PRIVATE_KEY")
    .parse::<LocalWallet>()?
    .with_chain_id(chain_id.as_u64());

    let wallet = SignerMiddleware::new
    (provider.clone(), private_key.clone());

    // TODO: Swap with real addresses
    let account1 = "0x0000000000000000000000000000000000000000".parse::<Address>()?;
    let account2 = "0x0000000000000000000000000000000000000000".parse::<Address>()?;

    // Balance before TX
    let bal1 = provider.get_balance(account1, None).await?;
    println!("Balance of account1 : {:?}", ethers::utils::format_ether(bal1));
    let bal2 = provider.get_balance(account2, None).await?;
    println!("Balance of account2 : {:?}\n", ethers::utils::format_ether(bal2));

    // Send transaction
    let tx = TransactionRequest::new()
        .to(account2.clone())
        .value(U256::from(ethers::utils::parse_ether(0.001)?));
    let tx = wallet.send_transaction(tx, None).await?.await?;

    println!("Receipt: {:?}", tx);

    // Balance after TX
    let bal1 = provider.get_balance(account1, None).await?;
    println!("\nBalance of account1 : {:?}", ethers::utils::format_ether(bal1));
    let bal2 = provider.get_balance(account2, None).await?;
    println!("Balance of account2 : {:?}", ethers::utils::format_ether(bal2));


    Ok(())
}