//! Benchmarking, shared between all embedded runtimes under `runtime/`, which run the same
//! checks on-device through the [`BENCHMARKS`] table.

#![cfg_attr(not(test), no_std)]

use epcc_keccak::{
    bitinterleaving::bitinterleaving_generic, conv_from_bi, conv_into_bi,
};

fn bench_conv_into_bi() {
    let r1 = core::hint::black_box(0x0123_4567u32);
    let r2 = core::hint::black_box(0x89ab_cdefu32);
    let _ = core::hint::black_box(conv_into_bi(r1, r2));
}

fn bench_generic_conv_into_bi() {
    let r1 = core::hint::black_box(0x0123_4567u32);
    let r2 = core::hint::black_box(0x89ab_cdefu32);
    let _ =
        core::hint::black_box(bitinterleaving_generic::conv_into_bi(r1, r2));
}

fn bench_conv_from_bi() {
    let r1 = core::hint::black_box(0x0123_4567u32);
    let r2 = core::hint::black_box(0x89ab_cdefu32);
    let _ = core::hint::black_box(conv_from_bi(r1, r2));
}

fn bench_generic_conv_from_bi() {
    let r1 = core::hint::black_box(0x0123_4567u32);
    let r2 = core::hint::black_box(0x89ab_cdefu32);
    let _ =
        core::hint::black_box(bitinterleaving_generic::conv_from_bi(r1, r2));
}

pub const BENCHMARKS: &[(&str, fn())] = &[
    ("conv_into_bi", bench_conv_into_bi),
    ("generic_conv_into_bi", bench_generic_conv_into_bi),
    ("conv_from_bi", bench_conv_from_bi),
    ("generic_conv_from_bi", bench_generic_conv_from_bi),
];
