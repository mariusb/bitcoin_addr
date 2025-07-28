#!/bin/bash
# filepath: /Users/mariusbock/LocalDocuments/Development/Rust/bitcoin_addr/view_found.sh

LOGFILE="${1:-bitcoin_balance.log}"

grep -E "Balance|^Start time:|^End time:|^Found" "$LOGFILE" | grep -v "Balance: 0 BTC" | grep "^Found"
echo "Filtered results from $LOGFILE:"
