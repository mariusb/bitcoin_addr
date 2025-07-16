#!/bin/bash
# filepath: /Users/mariusbock/LocalDocuments/Development/Rust/bitcoin_addr/view_balance.sh

grep -E "Balance|^Start time:" bitcoin_balance.log | grep -v "Balance: 0 BTC"
