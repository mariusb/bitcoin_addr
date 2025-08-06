#!/bin/bash
# filepath: /Users/mariusbock/LocalDocuments/Development/Rust/bitcoin_addr/test_summary.sh

# Only summarize and backup once per day
BACKUP_DATE=$(date +"%Y%m%d")
BACKUP_FILE="bitcoin_balance.log.$BACKUP_DATE"
SUMMARY_FILE="summary.log"

# Summarize "Found X addresses with balance, Y without balance"
if [ -f bitcoin_balance.log ]; then
    {
        echo "Summary for $(date +"%Y-%m-%d %H:%M:%S")"
        awk '
            /Found [0-9]+ addresses with balance, [0-9]+ without balance/ {
                # Split line by space and comma
                for (i=1; i<=NF; i++) {
                    if ($i == "Found") with += $(i+1)
                    if ($i == "with" && $(i+1) == "balance,") without += $(i+2)
                }
            }
            END {
                print "Total with balance:", with
                print "Total without balance:", without
            }
        ' bitcoin_balance.log
        echo ""
    } >> "$SUMMARY_FILE"
fi