#![allow(unused_imports)]
use bitcoin_address_generator::{
    generate_mnemonic, 
    WordCount, 
    derive_bitcoin_address, 
    derive_bitcoin_addresses,
    calculate_script_hash
};
use bitcoin::Network;

fn main() {
    /*  Generate a default 12-word mnemonic in English
    let mnemonic = generate_mnemonic(None, None).unwrap();
    println!("Generated mnemonic: {}", mnemonic);
    */

    // Generate a 24-word mnemonic in English
    let mnemonic = generate_mnemonic(Some(WordCount::Words24), None).unwrap();
    println!("24-word mnemonic: {}", mnemonic);

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
    println!("Legacy address: {}", p2pkh_addr.address);
    println!(" -> Legacy Script hash: {}", calculate_script_hash(p2pkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    
    // Derive a Nested SegWit (P2SH-WPKH) address
    let p2sh_wpkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/49'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None
    ).unwrap();
    println!("Nested SegWit address: {}", p2sh_wpkh_addr.address);
    println!(" -> Nested SegWit Script hash: {}", calculate_script_hash(p2sh_wpkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());
    
    // Derive a Native SegWit (P2WPKH) address
    let p2wpkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/84'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None
    ).unwrap();
    println!("Native SegWit address: {}", p2wpkh_addr.address);
    println!(" -> Native SegWit Script hash: {}", calculate_script_hash(p2wpkh_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());

    // Derive a Taproot (P2TR) address
    let p2tr_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/86'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None
    ).unwrap();
    println!("Taproot address: {}", p2tr_addr.address);
    println!(" -> Taproot Script hash: {}", calculate_script_hash(p2tr_addr.address.as_str(),Some(Network::Bitcoin)).unwrap());

    // Derive a testnet address
    let testnet_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/84'/1'/0'/0/0"),
        Some(Network::Testnet),
        None
    ).unwrap();
    println!("Testnet address: {}", testnet_addr.address);
    println!(" -> Testnet Script hash: {}", calculate_script_hash(testnet_addr.address.as_str(),Some(Network::Testnet)).unwrap());


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
    }
}