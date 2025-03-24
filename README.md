# Certora Verification Language for Rust (CVLR)

CVLR, pronounced "cavalier", is a set of Rust libraries that provide
verification primitives for Rust. We currently use it for writing formal specifications for Solana and Soroban smart contracts.
Examples of respective usage can be found [here](https://github.com/Certora/SolanaExamples) and [here](https://github.com/Certora/sunbeam-tutorials).
Refer to the Certora documentation for further information about the verification of [Solana](https://docs.certora.com/en/latest/docs/solana/index.html) and [Soroban](https://docs.certora.com/en/latest/docs/sunbeam/index.html) smart contracts.

## Building and Testing

To build the library, run `cargo build`, and to test the library, run `cargo test`.
For testing purposes, `cargo-expand` is required.
It can be installed by running the following command:

```bash
cargo install cargo-expand
```
