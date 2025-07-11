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
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct AddressInfo {
    chain_stats: ChainStats,
}

#[derive(Deserialize)]
struct ChainStats {
    funded_txo_sum: u64,
    spent_txo_sum: u64,
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
    // Generate a default 12-word mnemonic in English
    let mnemonic = generate_mnemonic(None, None).unwrap();
    println!("Generated mnemonic: {}", mnemonic);

    /*  Generate a 24-word mnemonic in English
    let mnemonic = generate_mnemonic(Some(WordCount::Words24), None).unwrap();
    println!("24-word mnemonic: {}", mnemonic);
    */
    /* Address derivation examples:
       - Legacy (P2PKH)
       - Nested SegWit (P2SH-WPKH)
       - Native SegWit (P2WPKH)
       - Taproot (P2TR)
       - Testnet addresses */
    // Derive a Legacy (P2PKH) address
    let p2pkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/44'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None
    ).unwrap();
    println!("Legacy address (P2PKH): {}", p2pkh_addr.address);
    // println!(" -> Legacy Script hash: {}", calculate_script_hash(p2pkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/44'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
    match btcbalance_from_mempool_space(p2pkh_addr.address.as_str()).await {
        Ok(balance) => if balance > 0.0 { 
            println!(" -> Balance: {} BTC <-------------- we have a winner!", balance) 
        } else { 
            println!(" -> Balance: {} BTC <-------------- what a let down", balance) 
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
    // println!(" -> Nested SegWit Script hash: {}", calculate_script_hash(p2sh_wpkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/49'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
    match btcbalance_from_mempool_space(p2sh_wpkh_addr.address.as_str()).await {
        Ok(balance) => if balance > 0.0 { 
            println!(" -> Balance: {} BTC <-------------- we have a winner!", balance) 
        } else { 
            println!(" -> Balance: {} BTC <-------------- what a let down", balance) 
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
    // println!(" -> Native SegWit Script hash: {}", calculate_script_hash(p2wpkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/84'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
    match btcbalance_from_mempool_space(p2wpkh_addr.address.as_str()).await {
        Ok(balance) => if balance > 0.0 { 
            println!(" -> Balance: {} BTC <-------------- we have a winner!", balance) 
        } else { 
            println!(" -> Balance: {} BTC <-------------- what a let down", balance) 
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
    // println!(" -> Taproot Script hash: {}", calculate_script_hash(p2tr_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    println!(" -> Private key (WIF): {}", derive_private_key(&mnemonic, Some("m/86'/0'/0'/0/0"), Some(Network::Bitcoin), None).unwrap());
    match btcbalance_from_mempool_space(p2tr_addr.address.as_str()).await {
        Ok(balance) => if balance > 0.0 { 
            println!(" -> Balance: {} BTC <-------------- we have a winner!", balance) 
        } else { 
            println!(" -> Balance: {} BTC <-------------- what a let down", balance) 
        },
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }

    /*  Derive a testnet address
    let testnet_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/84'/1'/0'/0/0"),
        Some(Network::Testnet),
        None
    ).unwrap();
    println!("Testnet address: {}", testnet_addr.address);
    println!(" -> Testnet Script hash: {}", calculate_script_hash(testnet_addr.address.as_str(),Some(Network::Testnet)).unwrap());
    */

    /* Some more examples of address derivation:
    // Derive 5 consecutive receiving addresses (m/86'/0'/0'/0/0 through m/86'/0'/0'/0/4) - Taproot addresses
    let receive_addresses = derive_bitcoin_addresses(
        &mnemonic,
        Some("m/86'/0'/0'"),  // Base path up to account level
        Some(Network::Bitcoin),
        None,                  // No BIP39 passphrase
        Some(false),           // Receiving addresses (false = receiving, true = change)
        Some(0),               // Start index
        Some(5)                // Number of addresses to generate
    ).unwrap();
    
    println!("Generated {} receiving addresses:", receive_addresses.addresses.len());
    for (i, addr) in receive_addresses.addresses.iter().enumerate() {
        println!("Address {}: {} (path: {})", i, addr.address, addr.path);
    }

    // Derive 3 change addresses (m/86'/0'/0'/1/0 through m/86'/0'/0'/1/2) - Taproot addresses
    let change_addresses = derive_bitcoin_addresses(
        &mnemonic,
        Some("m/86'/0'/0'"),  // Base path up to account level
        Some(Network::Bitcoin),
        None,                  // No BIP39 passphrase
        Some(true),            // Change addresses (false = receiving, true = change)
        Some(0),               // Start index
        Some(3)                // Number of addresses to generate
    ).unwrap();
    
    println!("\nGenerated {} change addresses:", change_addresses.addresses.len());
    for (i, addr) in change_addresses.addresses.iter().enumerate() {
        println!("Change Address {}: {} (path: {})", i, addr.address, addr.path);
    }
    
    // You can also start from a specific index - Taproot addresses
    // For example, to generate addresses starting from index 10:
    let custom_range = derive_bitcoin_addresses(
        &mnemonic,
        Some("m/86'/0'/0'"),
        Some(Network::Bitcoin),
        None,
        Some(false),
        Some(10),              // Start from index 10
        Some(2)                // Generate 2 addresses
    ).unwrap();
    
    println!("\nCustom range addresses:");
    for addr in custom_range.addresses.iter() {
        println!("{} (path: {})", addr.address, addr.path);
    } */
}