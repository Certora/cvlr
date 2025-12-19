# 🐕 Certora Verification Language for Rust (CVLR)

CVLR, pronounced "cavalier" 🐶, is a set of Rust libraries that provide verification primitives for Rust. We currently use it for writing formal specifications for Solana and Soroban smart contracts.

Examples of respective usage can be found in the [Solana Examples](https://github.com/Certora/SolanaExamples) and [Sunbeam Tutorials](https://github.com/Certora/sunbeam-tutorials) repositories.

Refer to the Certora documentation for further information about the verification of [Solana](https://docs.certora.com/en/latest/docs/solana/index.html) and [Soroban](https://docs.certora.com/en/latest/docs/sunbeam/index.html) smart contracts.

## Building and Testing

To build the library, run:

```bash
cargo build
```

To test the library, run:

```bash
cargo test
```

For testing purposes, `cargo-expand` is required. It can be installed by running:

```bash
cargo install cargo-expand
```

## Release

**Current release**: `0.4.2` 