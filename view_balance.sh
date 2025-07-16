#!/bin/bash
grep "Balance" bitcoin_balance.log | grep -v "Balance: 0 BTC"