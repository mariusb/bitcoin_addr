#!/bin/bash
../btc-addr-summary/target/release/btc-addr-summary
sqlite3 summary.db "SELECT * FROM summary;"
sqlite3 summary.db "SELECT sum(without_balance) FROM summary;"