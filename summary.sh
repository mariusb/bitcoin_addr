#!/bin/bash
../btc-addr-summary/target/release/btc-addr-summary
sqlite3 -header -column summary.db "SELECT * FROM summary;"
sqlite3 -header -column summary.db "SELECT sum(with_balance), sum(without_balance) FROM summary;"