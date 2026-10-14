//! Correctness tests and benchmarks for `epcc_keccak`, shared between
//! `cargo test` (via the `#[test]` fns below) and the embedded runtimes
//! under `runtime/`, which run the same checks on-device through the
//! plain-fn [`TESTS`] table.

#![cfg_attr(not(test), no_std)]
//mod bench;
mod sha3_32;
mod sha3_nominal;

// convenient: a single table the runners can iterate
pub const TESTS: &[(&str, fn())] = &[
    ("conversion", sha3_32::test_conv),
    ("sha3_KAT", sha3_nominal::test_sha3_known_answer),
];
