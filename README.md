![Build Status](https://img.shields.io/badge/simple-db-brightgreen)

## A simple user storing and retrieval script, built entirely on Rust using the Solana Program and Solana's Rust SDK.
Assuming you already have Solana cli installed.
### `simpledb` as current working directory:
- run `npm run clean`
- run `npm run build:program`
- run `solana program deploy dist/program/program.so`
- copy prompted program ID and enter it in load/main.rs Line43
- Modify first/last names as wished
- run `cargo build`
- run `cargo run`
- **WELCOME TO SOLANA!**

PS: Process of manually inserting the program ID and Keypair isn't the best approach. You probably should store your Keypair in a system file or somewhere safe!

// Already deployed program ID: **AxU4GMMQvSjuiTHFcA6U2HqRyAu1YpxaZ2imPPhnUMCa**
