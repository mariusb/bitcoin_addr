#!/bin/bash
# filepath: /Users/mariusbock/LocalDocuments/Development/Rust/bitcoin_addr/view_balance.sh

grep -E "Balance|^Start time:|^End time:|^Found" bitcoin_balance.log | grep -v "Balance: 0 BTC"
