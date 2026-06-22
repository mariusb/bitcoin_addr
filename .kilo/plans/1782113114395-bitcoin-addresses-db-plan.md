# Plan: Persist mnemonics and addresses to SQLite

## Goal
Add a local SQLite database (`./bitcoin_addresses.db`) that records every iteration's mnemonic, the 4 derived addresses, their WIF private keys, and the on-chain balance. The existing console output, the iteration argument, and the address types derived today must remain unchanged.

## Decisions (resolved with user)
- **Address types stored (4):** Legacy `P2PKH`, Nested SegWit `P2SH-WPKH`, Native SegWit `P2WPKH`, Taproot `P2TR` — the set the code already derives. The spec's duplicated "Native SegWit" is treated as a typo because the brief says "existing functionality should be kept as is."
- **DB file path:** `./bitcoin_addresses.db` resolved from the process current working directory.
- **DB write failures:** log to stderr and continue the loop (do not abort the run).
- **Referential integrity:** `addresses.mnemonic_id` is a `FOREIGN KEY ... REFERENCES mnemonics(id) ON DELETE CASCADE`, with `CREATE INDEX IF NOT EXISTS idx_addresses_mnemonic_id ON addresses(mnemonic_id)`, and `PRAGMA foreign_keys = ON` set on the connection.
- **Timestamps:** stored as local-time `DATE` and `TIME` text (`YYYY-MM-DD`, `HH:MM:SS`) via `chrono::Local`, matching the format already used in the console output.

## Data model

### `mnemonics` table
| Column     | Type    | Notes                                            |
|------------|---------|--------------------------------------------------|
| `id`       | INTEGER | `PRIMARY KEY AUTOINCREMENT`                      |
| `mnemonic` | TEXT    | 12-word BIP39 phrase                             |
| `date`     | TEXT    | Local date `YYYY-MM-DD` (SQLite `DATE` affinity) |
| `time`     | TEXT    | Local time `HH:MM:SS` (SQLite `TIME` affinity)   |

### `addresses` table
| Column         | Type    | Notes                                                  |
|----------------|---------|--------------------------------------------------------|
| `id`           | INTEGER | `PRIMARY KEY AUTOINCREMENT`                            |
| `mnemonic_id`  | INTEGER | `NOT NULL`, `FOREIGN KEY (mnemonic_id) REFERENCES mnemonics(id) ON DELETE CASCADE` |
| `address_type` | TEXT    | One of `P2PKH`, `P2SH-WPKH`, `P2WPKH`, `P2TR`          |
| `address`      | TEXT    | Derived Bitcoin address                                |
| `private_key`  | TEXT    | WIF-encoded private key                                |
| `balance`      | REAL    | Balance in BTC returned by mempool.space (or 0.0/`NULL` on error) |

Index: `CREATE INDEX IF NOT EXISTS idx_addresses_mnemonic_id ON addresses(mnemonic_id);`

## Database creation logic
1. Compute path: `std::env::current_dir()?.join("bitcoin_addresses.db")`.
2. Existence check is informational only; the schema is created idempotently on every run.
3. Open connection with `rusqlite::Connection::open(path)`.
4. Execute once at startup:
   - `PRAGMA foreign_keys = ON;`
   - `CREATE TABLE IF NOT EXISTS mnemonics (...);`
   - `CREATE TABLE IF NOT EXISTS addresses (...);`
   - `CREATE INDEX IF NOT EXISTS idx_addresses_mnemonic_id ON addresses(mnemonic_id);`
5. Wrap all per-iteration inserts in a single transaction (`connection.transaction()`); on error, log to stderr, roll back, and continue with the next iteration.

## New crate dependency
Add to `Cargo.toml`:
```
rusqlite = { version = "0.32", features = ["bundled"] }
```
- `bundled` avoids the system SQLite requirement and keeps the build self-contained.
- `chrono` (already present) supplies `Local::now()` for date/time.

No other crate changes required. `tokio`/`reqwest`/`bitcoin`/`bip39`/etc. remain untouched.

## Code changes (all in `src/main.rs`)

1. New imports: `rusqlite::{params, Connection}`, `std::path::PathBuf`, and a small `Db` helper struct.
2. Helper `fn open_db() -> rusqlite::Result<Connection>` that runs the `PRAGMA` and `CREATE` statements listed above.
3. Helper `fn insert_iteration(conn: &Connection, mnemonic: &str, entries: &[(type, address, wif, balance)])` that, in one transaction:
   - `INSERT INTO mnemonics (mnemonic, date, time) VALUES (?, ?, ?)` using `Local::now().format("%Y-%m-%d")` and `"%H:%M:%S"`.
   - Captures `last_insert_rowid` as `mnemonic_id`.
   - Inserts one row per address: `INSERT INTO addresses (mnemonic_id, address_type, address, private_key, balance) VALUES (?, ?, ?, ?, ?)`.
4. Refactor the per-iteration flow to:
   - Compute the private key once per address (current code calls `derive_private_key` separately; this is preserved behaviorally — the same value is produced).
   - Call the existing `btcbalance_from_mempool_space` once per address; reuse the returned `balance` for both console output and DB row.
   - Call `insert_iteration` after all four addresses have been processed. If the call returns `Err`, log via `eprintln!("DB write failed (iteration {}): {}", i + 1, e)` and continue.
5. Unchanged: argument parsing, `count`, `test_address` handling, console output text, found/not-found counters, start/end timestamps.
6. Optional: a short note in stderr if `./bitcoin_addresses.db` is newly created vs. reused (purely informational).

## Files affected
- `Cargo.toml` — add `rusqlite` dependency.
- `src/main.rs` — DB open, schema bootstrap, per-iteration insert, refactor to capture balance in a local variable.

## Steps to test
1. **Build:** `cargo build` succeeds with the new dependency.
2. **First run (new DB):**
   - `rm -f ./bitcoin_addresses.db && ./target/debug/bitcoin_addr 2`
   - Verify `./bitcoin_addresses.db` is created.
   - `sqlite3 ./bitcoin_addresses.db ".schema"` shows both tables and the index.
   - `sqlite3 ./bitcoin_addresses.db "SELECT COUNT(*) FROM mnemonics;"` returns `2`; `SELECT COUNT(*) FROM addresses;` returns `8`.
   - Spot-check a row: `SELECT address_type, address, balance FROM addresses LIMIT 4;` — values match what was printed to console.
3. **Second run (existing DB):**
   - Re-run `./target/debug/bitcoin_addr 2`.
   - Row counts in both tables should be `4` mnemonics / `16` addresses (additive; no `id` collisions, no schema errors).
4. **FK enforcement:** `sqlite3 ./bitcoin_addresses.db "PRAGMA foreign_key_list(addresses);"` shows a FK to `mnemonics.id` with `ON DELETE CASCADE`.
5. **DB write failure path:** temporarily make the file read-only (`chmod 444 ./bitcoin_addresses.db`) and re-run; the program must log the error to stderr and still print the iteration's console output, then exit with the normal summary.
6. **Console parity:** diff the new console output against a baseline run with the same seed-like arguments (the printed lines and ordering must match the pre-change output).
7. **Lint/typecheck:** `cargo clippy --all-targets -- -D warnings` and `cargo build --release` both succeed.

## Assumptions and caveats
- `balance` is the same value the program already prints, which comes from mempool.space's `chain_stats` (confirmed UTXOs only — does not include unconfirmed/mempool balances). Storing this preserves parity with the console output but is not a true "current" balance in the strictest sense.
- The database grows monotonically; no retention/cleanup is added in this change.
- The DB is opened in the process's current working directory. If the user runs the binary from a different directory, a new `bitcoin_addresses.db` is created there.
- `rusqlite` is used in synchronous mode inside the existing `#[tokio::main]` async runtime; per-iteration DB writes are small and will not noticeably affect throughput, but they do add a small synchronous cost to each iteration.
- The 4 address types are derived with the existing derivation paths: `m/44'/0'/0'/0/0` (P2PKH), `m/49'/0'/0'/0/0` (P2SH-WPKH), `m/84'/0'/0'/0/0` (P2WPKH), `m/86'/0'/0'/0/0` (P2TR).
- Private keys are stored in WIF as returned by `derive_private_key`. The DB file is not encrypted; this matches the current exposure of private keys in console output but is worth noting for future hardening.
- The pre-existing `summary.db` in the repo root is unrelated to this change and is not touched.
