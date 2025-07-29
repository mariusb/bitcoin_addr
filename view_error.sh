#!/bin/bash
# filepath: /Users/mariusbock/LocalDocuments/Development/Rust/bitcoin_addr/view_error.sh

LOGFILE="${1:-bitcoin_balance.log}"

grep -E "Error|Failed" "$LOGFILE"
echo "Filtered results from $LOGFILE:"
