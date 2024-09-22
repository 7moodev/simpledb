![Build Status](https://img.shields.io/badge/simple-db-brightgreen)

## A simple user storing and retrieval script, built entirely on Rust using the Solana Program and Solana's Rust SDK.
Assuming you already have Solana cli installed:
- set `/simpledb` as current working directory
- run `npm run clean`
- run `npm run build:program`
- run `solana program deploy dist/program/program.so`
- copy prompted program ID and enter it in load/main.rs #line43
- set `/load` as current working directory
- run `cargo build`
- run `cargo run`
