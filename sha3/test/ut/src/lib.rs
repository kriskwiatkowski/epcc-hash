#![cfg_attr(not(test), no_std)]

use epcc_keccak::{
    bitinterleaving::bitinterleaving_generic, conv_from_bi, conv_into_bi,
};

const TEST_INPUTS: [u64; 6] = [
    0,
    1,
    0x0123_4567_89ab_cdef,
    0x5555_5555_5555_5555,
    0xaaaa_aaaa_aaaa_aaaa,
    u64::MAX,
];

// Same as bitinterleaving_generic::conv_into_bi, but smaller
fn shuffle_even(x: u64) -> u32 {
    let mut x = x & 0x5555_5555_5555_5555;
    x = (x | (x >> 1)) & 0x3333_3333_3333_3333;
    x = (x | (x >> 2)) & 0x0f0f_0f0f_0f0f_0f0f;
    x = (x | (x >> 4)) & 0x00ff_00ff_00ff_00ff;
    x = (x | (x >> 8)) & 0x0000_ffff_0000_ffff;
    x = (x | (x >> 16)) & 0x0000_0000_ffff_ffff;
    x as u32
}

fn shuffle_odd(x: u64) -> u32 {
    shuffle_even(x >> 1)
}

fn test_conv_into_bi() {
    for input in TEST_INPUTS {
        let expected = (shuffle_even(input), shuffle_odd(input));

        let r1 = (input >> 0x00) as u32;
        let r2 = (input >> 0x20) as u32;
        let actual: (u32, u32) = conv_into_bi(r1, r2);
        if actual != expected {
            panic!("actual {actual:?} did not equal expected {expected:?}");
        }
    }
}

fn test_conv_from_bi() {
    for input in TEST_INPUTS {
        let expected = (input as u32, (input >> 32) as u32);

        let r1 = shuffle_even(input);
        let r2 = shuffle_odd(input);
        let actual = conv_from_bi(r1, r2);
        if actual != expected {
            panic!("actual {actual:?} did not equal expected {expected:?}");
        }
    }
}

#[cfg(test)]
#[test]
fn conv_into_bi_matches_reference() {
    test_conv_into_bi();
}

#[cfg(test)]
#[test]
fn conv_from_bi_matches_reference() {
    test_conv_from_bi();
}

// convenient: a single table the runners can iterate
pub const TESTS: &[(&str, fn())] =
    &[("conv_into_bi", test_conv_into_bi), ("conv_from_bi", test_conv_from_bi)];

// Benchmarks

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
