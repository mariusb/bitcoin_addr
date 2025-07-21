#!/bin/bash

# Backup bitcoin_balance.log at the first run (if not already backed up this session)
BACKUP_DATE=$(date +"%Y%m%d")
BACKUP_FILE="bitcoin_balance.log.$BACKUP_DATE"

if [ ! -f "$BACKUP_FILE" ]; then
    if [ -f bitcoin_balance.log ]; then
        mv bitcoin_balance.log "$BACKUP_FILE"
        echo "Backup created: $BACKUP_FILE" >> bitcoin_balance.log 2>&1
    fi
fi

target/release/bitcoin_addr 8 >> bitcoin_balance.log 2>&1