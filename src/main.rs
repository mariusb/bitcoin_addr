#![allow(unused_imports)]
use bitcoin_address_generator::{
    generate_mnemonic, 
    WordCount, 
    derive_bitcoin_address, 
    derive_bitcoin_addresses,
    calculate_script_hash,
    derive_private_key
};
use bitcoin::Network;
use reqwest::Error;
use reqwest;
use serde::Deserialize;
use std::env;
use chrono::Local;

#[derive(Deserialize)]
struct AddressInfo {
    chain_stats: ChainStats,
}

#[derive(Deserialize)]
struct ChainStats {
    funded_txo_sum: u64,
    spent_txo_sum: u64,
}

#[derive(Debug, Deserialize)]
struct Tx {
    vout: Vec<Vout>,
}

#[derive(Debug, Deserialize)]
struct Vout {
    scriptpubkey_address: Option<String>,
}

async fn get_first_output_address_from_latest_block() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // 1. Get the latest block hash
    let latest_block_hash: String = client
        .get("https://mempool.space/api/blocks/tip/hash")
        .send()
        .await?
        .text()
        .await?;

    // 2. Get the transaction IDs for the latest block
    let txids_url = format!("https://mempool.space/api/block/{}/txids", latest_block_hash);
    let txids: Vec<String> = client.get(&txids_url).send().await?.json().await?;

    if txids.is_empty() {
        return Err("No transactions in the latest block.".into());
    }

    // 3. Get the first transaction
    let first_txid = &txids[0];

    // 4. Get the details of the first transaction
    let tx_url = format!("https://mempool.space/api/tx/{}", first_txid);
    let tx: Tx = client.get(&tx_url).send().await?.json().await?;

    // 5. Find the first output address
    if let Some(first_vout) = tx.vout.get(0) {
        if let Some(address) = &first_vout.scriptpubkey_address {
            return Ok(address.clone());
        } else {
            return Err("First output has no address.".into());
        }
    } else {
        return Err("Transaction has no outputs.".into());
    }
}

async fn btcbalance_from_mempool_space(address: &str) -> Result<f64, Error> {
    // Construct the mempool.space API URL
    let url = format!("https://mempool.space/api/address/{}", address);

    // Make the HTTP request
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    // Parse the JSON response
    let address_info: AddressInfo = response.json().await?;

    // Calculate balance in satoshis
    let balance = address_info.chain_stats.funded_txo_sum - address_info.chain_stats.spent_txo_sum;

    // Convert to BTC (1 BTC = 100,000,000 satoshis)
    Ok(balance as f64 / 100_000_000.0)
}

#[tokio::main]
async fn main() {
    // Print date and time stamp
    println!("Start time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    // Read arguments: first is number of iterations, second is a bitcoin address
    let args: Vec<String> = std::env::args().collect();
    let count: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };
    let test_address = if args.len() > 2 {
        args[2].clone()
    } else {
        get_first_output_address_from_latest_block().await.unwrap_or_else(|_| {
            eprintln!("Failed to get the first output address from the latest block.");
            "bc1qq5552m27lql80chjze0d8pty0r4dfezeucymkd".to_string() // Default address if fetching fails
        })
    };
    
    println!("Test address with a balance: {}", test_address);
    match btcbalance_from_mempool_space(test_address.as_str()).await {
        Ok(balance) => if balance > 0.0 { 
            println!(" -> Balance: {} BTC <-------------- we have a winner! -- Address: {}", balance, test_address) 
        } else { 
            println!(" -> Balance: {} BTC <-------------- what a let down", balance) 
        },
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }
    let mut count_found = 0;
    let mut count_not_found = 0;

    for i in 0..count {
        println!("\n=== Iteration {} ===", i + 1);

        // Generate a default 12-word mnemonic in English
        let mnemonic = generate_mnemonic(None, None).unwrap();
        println!("Generated mnemonic: {}", mnemonic);

        // Derive a Legacy (P2PKH) address
        let p2pkh_addr = derive_bitcoin_address(
            &mnemonic,
            Some("m/44'/0'/0'/0/0"),
            Some(Network::Bitcoin),
            None
        ).unwrap();
        println!("Legacy address (P2PKH): {}", p2pkh_addr.address);
        println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/44'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
        match btcbalance_from_mempool_space(p2pkh_addr.address.as_str()).await {
            Ok(balance) => if balance > 0.0 { 
                println!(" -> Balance: {} BTC <-------------- we have a winner! -- Address: {}", balance, p2pkh_addr.address);
                count_found += 1; 
            } else { 
                println!(" -> Balance: {} BTC <-------------- what a let down", balance);
                count_not_found += 1;
            },
            Err(e) => eprintln!("Error fetching balance: {}", e),
        }

        // Derive a Nested SegWit (P2SH-WPKH) address
        let p2sh_wpkh_addr = derive_bitcoin_address(
            &mnemonic,
            Some("m/49'/0'/0'/0/0"),
            Some(Network::Bitcoin),
            None
        ).unwrap();
        println!("Nested SegWit address (P2SH-WPKH): {}", p2sh_wpkh_addr.address);
        println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/49'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
        match btcbalance_from_mempool_space(p2sh_wpkh_addr.address.as_str()).await {
            Ok(balance) => if balance > 0.0 { 
                println!(" -> Balance: {} BTC <-------------- we have a winner! -- Address: {}", balance, p2sh_wpkh_addr.address);
                count_found += 1;
            } else { 
                println!(" -> Balance: {} BTC <-------------- what a let down", balance);
                count_not_found += 1;
            },
            Err(e) => eprintln!("Error fetching balance: {}", e),
        }

        // Derive a Native SegWit (P2WPKH) address
        let p2wpkh_addr = derive_bitcoin_address(
            &mnemonic,
            Some("m/84'/0'/0'/0/0"),
            Some(Network::Bitcoin),
            None
        ).unwrap();
        println!("Native SegWit address (P2WPKH): {}", p2wpkh_addr.address);
        println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/84'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
        match btcbalance_from_mempool_space(p2wpkh_addr.address.as_str()).await {
            Ok(balance) => if balance > 0.0 { 
                println!(" -> Balance: {} BTC <-------------- we have a winner! -- Address: {}", balance, p2wpkh_addr.address);
                count_found += 1;
            } else { 
                println!(" -> Balance: {} BTC <-------------- what a let down", balance);
                count_not_found += 1;
            },
            Err(e) => eprintln!("Error fetching balance: {}", e),
        }

        // Derive a Taproot (P2TR) address
        let p2tr_addr = derive_bitcoin_address(
            &mnemonic,
            Some("m/86'/0'/0'/0/0"),
            Some(Network::Bitcoin),
            None
        ).unwrap();
        println!("Taproot address (P2TR): {}", p2tr_addr.address);
        println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/86'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
        match btcbalance_from_mempool_space(p2tr_addr.address.as_str()).await {
            Ok(balance) => if balance > 0.0 { 
                println!(" -> Balance: {} BTC <-------------- we have a winner! -- Address: {}", balance, p2tr_addr.address);
                count_found += 1;
            } else { 
                println!(" -> Balance: {} BTC <-------------- what a let down", balance);
                count_not_found += 1;
            },
            Err(e) => eprintln!("Error fetching balance: {}", e),
        }
    }
    println!("Found {} addresses with balance, {} without balance", count_found, count_not_found);
    println!("End time: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));
}