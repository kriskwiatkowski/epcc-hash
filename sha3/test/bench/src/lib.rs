//! Benchmarking, shared between all embedded runtimes under `runtime/`, which run the same
//! checks on-device through the [`BENCHMARKS`] table.
//!
//! Each entry takes an accumulator and returns one derived from its
//! result (`r1 ^ r2`), rather than taking fixed inputs and discarding
//! the output. A caller that threads the accumulator through repeated
//! calls (`acc = f(acc)`) creates a real dependency from one call to
//! the next, so the measured time reflects one call's critical-path
//! latency rather than the throughput of many independent calls
//! overlapping on an out-of-order CPU.

#![cfg_attr(not(test), no_std)]

use epcc_keccak::{
    bitinterleaving::bitinterleaving_generic, conv_from_bi, conv_into_bi,
    sha3::sha3_256,
};

fn bench_conv_into_bi(acc: u32) -> u32 {
    let r1 = core::hint::black_box(acc);
    let r2 = core::hint::black_box(acc.wrapping_add(1));
    let (s1, s2) = core::hint::black_box(conv_into_bi(r1, r2));
    s1 ^ s2
}

fn bench_generic_conv_into_bi(acc: u32) -> u32 {
    let r1 = core::hint::black_box(acc);
    let r2 = core::hint::black_box(acc.wrapping_add(1));
    let (s1, s2) =
        core::hint::black_box(bitinterleaving_generic::conv_into_bi(r1, r2));
    s1 ^ s2
}

fn bench_conv_from_bi(acc: u32) -> u32 {
    let r1 = core::hint::black_box(acc);
    let r2 = core::hint::black_box(acc.wrapping_add(1));
    let (o1, o2) = core::hint::black_box(conv_from_bi(r1, r2));
    o1 ^ o2
}

fn bench_generic_conv_from_bi(acc: u32) -> u32 {
    let r1 = core::hint::black_box(acc);
    let r2 = core::hint::black_box(acc.wrapping_add(1));
    let (o1, o2) =
        core::hint::black_box(bitinterleaving_generic::conv_from_bi(r1, r2));
    o1 ^ o2
}

fn bench_sha3_256(acc: u32) -> u32 {
    let digest = sha3_256(b"msg");
    acc + acc + digest[0] as u32
}

pub const BENCHMARKS: &[(&str, fn(u32) -> u32)] = &[
    ("sha3_256", bench_sha3_256),
    ("conv_into_bi", bench_conv_into_bi),
    ("generic_conv_into_bi", bench_generic_conv_into_bi),
    ("conv_from_bi", bench_conv_from_bi),
    ("generic_conv_from_bi", bench_generic_conv_from_bi),
];
