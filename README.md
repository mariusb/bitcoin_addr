# Bitcoin Address Hunter

This project is a simple tool for generating random Bitcoin addresses and checking if they have a balance. It's an exploration of Bitcoin address generation and blockchain data retrieval. The tool generates a mnemonic phrase, derives multiple types of addresses (Legacy, SegWit, Taproot), and then checks their balance using the mempool.space API.

**Disclaimer:** The probability of finding a Bitcoin address with a non-zero balance is astronomically low. This tool is for educational and experimental purposes only. Do not expect to find any bitcoins with it.

## Features

- Generates a 12-word BIP39 mnemonic phrase.
- Derives multiple Bitcoin address types from the mnemonic:
  - Legacy (P2PKH)
  - Nested SegWit (P2SH-WPKH)
  - Native SegWit (P2WPKH)
  - Taproot (P2TR)
- Derives the corresponding private key for each address in Wallet Import Format (WIF).
- Checks the balance of each generated address using the `mempool.space` public API.
- A suite of helper scripts to automate execution and log viewing.

## How to Build

To compile the application, you need to have the Rust toolchain installed. You can find instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can build the project with the following command:

```bash
cargo build --release
```

The compiled binary will be located at `target/release/bitcoin_addr`.

## Usage

There are two main ways to use this tool: running the executable directly or using the provided helper scripts.

### Direct Execution

You can run the compiled program directly from the command line.

```bash
./target/release/bitcoin_addr [iterations] [test_address]
```

-   `[iterations]` (optional): The number of random mnemonics to generate and check. Defaults to 1.
-   `[test_address]` (optional): A specific Bitcoin address you want to check the balance of.

### Using the Helper Scripts

The most convenient way to run the tool is by using the `check_balance.sh` script. This script runs the `bitcoin_addr` program for 8 iterations and saves the output to `bitcoin_balance.log`. It also handles backing up the log file daily.

```bash
./check_balance.sh
```

To make monitoring the logs easier, you can use the `set_alias.sh` script to create some helpful command-line aliases.

```bash
source ./set_alias.sh
```

Now you can use `vb` to view balances, `ve` to view errors, and `vf` to view a summary of found balances.

## Scripts

This project includes several helper scripts to automate the process of running the tool and viewing the results.

-   `check_balance.sh`: The main script for running the `bitcoin_addr` program. It runs the program for 8 iterations, appends the output to `bitcoin_balance.log`, and creates a daily backup of the log file.
-   `set_alias.sh`: Sets up convenient command-line aliases for the `view_*.sh` scripts:
    -   `vb`: alias for `./view_balance.sh`
    -   `ve`: alias for `./view_error.sh`
    -   `vf`: alias for `./view_found.sh`
-   `view_balance.sh`: Displays any lines from the log file that show a non-zero balance.
-   `view_error.sh`: Displays any error messages that have been logged.
-   `view_found.sh`: Shows a summary of the number of addresses found with a balance.
-   `test_summary.sh`: A script to manually trigger a summary of the current `bitcoin_balance.log` file without rotating the log. The summary is appended to `summary.log`.
