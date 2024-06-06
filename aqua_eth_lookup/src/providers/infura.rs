use dotenv::dotenv;
use ethers::{
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
};
use eyre::{Report, Result, WrapErr};
use serde::Deserialize;
use serde_json::from_value;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct CustomTransaction {
    blockNumber: String,
    input: String,
}

#[derive(Deserialize, Debug)]
struct Blocktime {
    timestamp: String,
}

// This clode does eth_look of the Guardian components
// Using the infura service for Ethereum API calls https://www.infura.io/
// This code should support ethereum sepolia, holesky and mainnet networks
// This code should be able to get the transaction details from the network via the transaction hash
// Interface should be async fn lookup(ethereum_chain_id, tx_hash) -> (timestamp, event_hash)

//Parse the received data into serde_json
//Provide an internal interface

// Test-data inputs to verify functionality of the function
// (mainnet, 0x9d4897d3e381982ee872cb193469d991cce8d087f0cd5fe275926f80c1326a1e)
// (holesky, 0xe20ee33fe150423099d6c22bf84683e19d03e40371e2c76e59293d026e8d0101)
// (sepolia, 0xae9b476d8eed73897b0f71ac59c267856dbae64f249518fea862377208436cc5)

fn chain_id_to_url() -> HashMap<u32, &'static str> {
    let mut map = HashMap::new();
    map.insert(0x1, "https://mainnet.infura.io/v3/");
    map.insert(0x4268, "https://holesky.infura.io/v3/");
    map.insert(0xaa36a7, "https://sepolia.infura.io/v3/");
    map
}

pub async fn get_tx_data(chain_id: u32, tx_hash: &str) -> Result<(H512, u64), Report> {
    // Load the .env file
    dotenv::dotenv().ok();

    // Load Infura API key from .env file
    let infura_api_key = std::env::var("INFURA_API_KEY").wrap_err("INFURA_API_KEY must be set")?;

    // Get the URL prefix for the specified chain ID
    let urls = chain_id_to_url();
    let url_prefix = urls
        .get(&chain_id)
        .ok_or_else(|| eyre::eyre!("Unsupported chain ID: {}", chain_id))?;

    // Build the full URL
    let url = format!("{}{}", url_prefix, infura_api_key);

    // Connect to the network via Infura
    let provider = Provider::<Http>::try_from(url).wrap_err("Failed to create provider")?;

    // Get the chain ID
    let chain_id = provider
        .get_chainid()
        .await
        .wrap_err("Failed to get chain ID")?;

    // Load the signer private key from the .env file
    let wallet_key =
        std::env::var("SIGNER_PRIVATE_KEY").wrap_err("SIGNER_PRIVATE_KEY must be set")?;

    // Parse the wallet key
    let wallet: LocalWallet = wallet_key
        .parse::<LocalWallet>()
        .wrap_err("Failed to parse wallet key")?
        .with_chain_id(chain_id.as_u64());

    // Connect the wallet to the provider
    let client = SignerMiddleware::new(provider, wallet);

    // Parse the transaction hash
    let transaction_hash: H256 = tx_hash
        .parse()
        .wrap_err("Failed to parse transaction hash")?;

    // Get the transaction
    let tx = client
        .get_transaction(transaction_hash)
        .await
        .wrap_err("Failed to get transaction")?
        .ok_or_else(|| eyre::eyre!("Transaction not found"))?;

    // Deserialize the transaction
    let tx: CustomTransaction =
        from_value(serde_json::to_value(&tx)?).wrap_err("Failed to deserialize transaction")?;

    // Parse the block number
    let blocknumber = u64::from_str_radix(tx.blockNumber.trim_start_matches("0x"), 16)
        .wrap_err("Failed to parse block number")?;

    // Get the block
    let block = client
        .get_block(blocknumber)
        .await
        .wrap_err("Failed to get block")?
        .ok_or_else(|| eyre::eyre!("Block not found"))?;

    // Deserialize the block time
    let blocktime: Blocktime =
        from_value(serde_json::to_value(&block)?).wrap_err("Failed to deserialize block time")?;

    // Parse the block timestamp
    let blocktime_u64 = u64::from_str_radix(blocktime.timestamp.trim_start_matches("0x"), 16)
        .wrap_err("Failed to parse timestamp")?;

    // Parse the input
    let input = tx.input[10..]
        .parse::<H512>()
        .wrap_err("Failed to parse input")?;

    Ok((input, blocktime_u64))
}
