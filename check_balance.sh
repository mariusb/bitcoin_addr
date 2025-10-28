#!/bin/bash

# Backup bitcoin_balance.log at the first run (if not already backed up this session)
BACKUP_DATE=$(date +"%Y%m%d")
BACKUP_FILE="bitcoin_balance.log.$BACKUP_DATE"
SUMMARY_FILE="summary.log"

if [ ! -f "$BACKUP_FILE" ]; then
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
        } >> "$SUMMARY_FILE" 2>&1
        mv bitcoin_balance.log "$BACKUP_FILE"
        echo "Backup created: $BACKUP_FILE" >> bitcoin_balance.log 2>&1
    fi
fi

target/release/bitcoin_addr 10 >> bitcoin_balance.log 2>&1