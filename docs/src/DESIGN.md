# Design

This page explains how `epcc-hash` is put together for people modifying the crate rather than consuming it.

## Crate layout
The workspace is a Cargo workspace with the following members:

### The library
- `sha3` (`epcc-keccak`) — the main library crate containing the Keccak/SHA3 implementation.
- `sha3/test` (`keccak-test`) — unit-test/benchmark crate, shared between `cargo test` and embedded runtimes.
- `sha3/test/kat` (`keccak-kat`) — known-answer test crate, driven by `cargo xtask kat`.

## Feature flags

| Feature      | Effect |
|--------------|--------|
| `std`        | Lets the crate use `std` instead of `no_std` facilities. This is required for unit testing on the host. |
| `keccak32bit`| Selects the `keccak_32` backend instead of the default `keccak_64`. Pick this manually per build/target — it isn't auto-detected. |
| `testing`    | Re-exports some internal modules, which makes unit testing possible. See [QA](qa.md) for more details. |

### Build system helper

Project uses `xtask` to run build-system helper tasks. Those include documentation generation or testing against KAT vectors.

For example, when it comes to KAT testing, the project uses `xtask` in order to download KAT vectors, download a KAT parser (`katwalk` crate)) and run the tests against the implementation.


## Build-time code generation

Project uses code generation for assembly optimized code. This is done by the `sha3/build/build.rs` script, that runs `gen/bitinterleaving.py` at build time to (re)generate `src/bitinterleaving.rs`'s. It reruns only when the script changes (`cargo:rerun-if-changed`). If you need to change the bit-interleaving tables, edit the Python generator, not the generated file.
