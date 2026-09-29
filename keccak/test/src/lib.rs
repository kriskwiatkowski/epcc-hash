#![cfg_attr(not(test), no_std)]

use epcc_keccak::{
    bitinterleaving::bitinterleaving_generic, conv_from_bi, conv_into_bi,
    keccak_f1600, keccak_f1600_generic, sha3_224, sha3_256, sha3_384, sha3_512,
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

// NIST FIPS 202 known-answer values.
const SHA3_224_EMPTY: [u8; 28] = [
    0x6b, 0x4e, 0x03, 0x42, 0x36, 0x67, 0xdb, 0xb7, 0x3b, 0x6e, 0x15, 0x45,
    0x4f, 0x0e, 0xb1, 0xab, 0xd4, 0x59, 0x7f, 0x9a, 0x1b, 0x07, 0x8e, 0x3f,
    0x5b, 0x5a, 0x6b, 0xc7,
];
const SHA3_256_EMPTY: [u8; 32] = [
    0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56,
    0xa0, 0x61, 0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa,
    0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a,
];
const SHA3_384_EMPTY: [u8; 48] = [
    0x0c, 0x63, 0xa7, 0x5b, 0x84, 0x5e, 0x4f, 0x7d, 0x01, 0x10, 0x7d, 0x85,
    0x2e, 0x4c, 0x24, 0x85, 0xc5, 0x1a, 0x50, 0xaa, 0xaa, 0x94, 0xfc, 0x61,
    0x99, 0x5e, 0x71, 0xbb, 0xee, 0x98, 0x3a, 0x2a, 0xc3, 0x71, 0x38, 0x31,
    0x26, 0x4a, 0xdb, 0x47, 0xfb, 0x6b, 0xd1, 0xe0, 0x58, 0xd5, 0xf0, 0x04,
];
const SHA3_512_EMPTY: [u8; 64] = [
    0xa6, 0x9f, 0x73, 0xcc, 0xa2, 0x3a, 0x9a, 0xc5, 0xc8, 0xb5, 0x67, 0xdc,
    0x18, 0x5a, 0x75, 0x6e, 0x97, 0xc9, 0x82, 0x16, 0x4f, 0xe2, 0x58, 0x59,
    0xe0, 0xd1, 0xdc, 0xc1, 0x47, 0x5c, 0x80, 0xa6, 0x15, 0xb2, 0x12, 0x3a,
    0xf1, 0xf5, 0xf9, 0x4c, 0x11, 0xe3, 0xe9, 0x40, 0x2c, 0x3a, 0xc5, 0x58,
    0xf5, 0x00, 0x19, 0x9d, 0x95, 0xb6, 0xd3, 0xe3, 0x01, 0x75, 0x85, 0x86,
    0x28, 0x1d, 0xcd, 0x26,
];
const SHA3_256_ABC: [u8; 32] = [
    0x3a, 0x98, 0x5d, 0xa7, 0x4f, 0xe2, 0x25, 0xb2, 0x04, 0x5c, 0x17, 0x2d,
    0x6b, 0xd3, 0x90, 0xbd, 0x85, 0x5f, 0x08, 0x6e, 0x3e, 0x9d, 0x52, 0x5b,
    0x46, 0xbf, 0xe2, 0x45, 0x11, 0x43, 0x15, 0x32,
];

fn test_sha3_224_empty() {
    let actual = sha3_224(&[]);
    if actual != SHA3_224_EMPTY {
        panic!(
            "actual {actual:02x?} did not equal expected {SHA3_224_EMPTY:02x?}"
        );
    }
}

fn test_sha3_256_empty() {
    let actual = sha3_256(&[]);
    if actual != SHA3_256_EMPTY {
        panic!(
            "actual {actual:02x?} did not equal expected {SHA3_256_EMPTY:02x?}"
        );
    }
}

fn test_sha3_384_empty() {
    let actual = sha3_384(&[]);
    if actual != SHA3_384_EMPTY {
        panic!(
            "actual {actual:02x?} did not equal expected {SHA3_384_EMPTY:02x?}"
        );
    }
}

fn test_sha3_512_empty() {
    let actual = sha3_512(&[]);
    if actual != SHA3_512_EMPTY {
        panic!(
            "actual {actual:02x?} did not equal expected {SHA3_512_EMPTY:02x?}"
        );
    }
}

fn test_sha3_256_abc() {
    let actual = sha3_256(b"abc");
    if actual != SHA3_256_ABC {
        panic!(
            "actual {actual:02x?} did not equal expected {SHA3_256_ABC:02x?}"
        );
    }
}

// Exercises absorbing across multiple sponge blocks and buffered
// `update` calls, not just a single padded block.
fn test_sha3_256_multi_block() {
    use epcc_keccak::Sha3_256;

    let data = [0x61u8; 1_000]; // 1000 'a's, well past the 136-byte rate
    let expected = sha3_256(&data);

    let mut h = Sha3_256::new();
    for chunk in data.chunks(37) {
        h.update(chunk);
    }
    let actual = h.finalize();

    if actual != expected {
        panic!("actual {actual:02x?} did not equal expected {expected:02x?}");
    }
}

#[cfg(test)]
#[test]
fn sha3_224_empty_matches_reference() {
    test_sha3_224_empty();
}

#[cfg(test)]
#[test]
fn sha3_256_empty_matches_reference() {
    test_sha3_256_empty();
}

#[cfg(test)]
#[test]
fn sha3_384_empty_matches_reference() {
    test_sha3_384_empty();
}

#[cfg(test)]
#[test]
fn sha3_512_empty_matches_reference() {
    test_sha3_512_empty();
}

#[cfg(test)]
#[test]
fn sha3_256_abc_matches_reference() {
    test_sha3_256_abc();
}

#[cfg(test)]
#[test]
fn sha3_256_multi_block_matches_oneshot() {
    test_sha3_256_multi_block();
}

// Keccak-f[1600] applied to the all-zero state; the canonical vector
// from the Keccak reference code.
const F1600_ZERO_OUT: [u64; 25] = [
    0xf1258f7940e1dde7,
    0x84d5ccf933c0478a,
    0xd598261ea65aa9ee,
    0xbd1547306f80494d,
    0x8b284e056253d057,
    0xff97a42d7f8e6fd4,
    0x90fee5a0a44647c4,
    0x8c5bda0cd6192e76,
    0xad30a6f71b19059c,
    0x30935ab7d08ffc64,
    0xeb5aa93f2317d635,
    0xa9a6e6260d712103,
    0x81a57c16dbcf555f,
    0x43b831cd0347c826,
    0x01f22f1a11a5569f,
    0x05e5635a21d9ae61,
    0x64befef28cc970f2,
    0x613670957bc46611,
    0xb87c5a554fd00ecb,
    0x8c3ee88a1ccf32c8,
    0x940c7922ae3a2614,
    0x1841f924a2c509e4,
    0x16f53526e70465c2,
    0x75f644e97f30a13b,
    0xeaf1ff7b5ceca249,
];

fn test_keccak_f1600_zero() {
    let mut state = [0u64; 25];
    keccak_f1600(&mut state);
    if state != F1600_ZERO_OUT {
        panic!("keccak_f1600 of the zero state did not match the KAT");
    }
}

// Guards the architecture-specific permutations against the portable
// one on the target's own hardware.
fn test_keccak_f1600_matches_generic() {
    // A cheap deterministic spread of lane values; no PRNG needed.
    let mut state = [0u64; 25];
    let mut v = 0x0123_4567_89ab_cdefu64;
    for lane in state.iter_mut() {
        *lane = v;
        v = v.rotate_left(7) ^ 0x9e37_79b9_7f4a_7c15;
    }
    let mut expected = state;
    keccak_f1600_generic(&mut expected);
    keccak_f1600(&mut state);
    if state != expected {
        panic!("keccak_f1600 disagreed with keccak_f1600_generic");
    }

    // Iterating catches state-dependent bugs a single call can miss.
    let mut a = [0u64; 25];
    let mut b = [0u64; 25];
    for _ in 0..8 {
        keccak_f1600(&mut a);
        keccak_f1600_generic(&mut b);
        if a != b {
            panic!("keccak_f1600 diverged from generic under iteration");
        }
    }
}

#[cfg(test)]
#[test]
fn keccak_f1600_zero_matches_reference() {
    test_keccak_f1600_zero();
}

#[cfg(test)]
#[test]
fn keccak_f1600_agrees_with_generic() {
    test_keccak_f1600_matches_generic();
}

// convenient: a single table the runners can iterate
pub const TESTS: &[(&str, fn())] = &[
    ("conv_into_bi", test_conv_into_bi),
    ("conv_from_bi", test_conv_from_bi),
    ("keccak_f1600_zero", test_keccak_f1600_zero),
    ("keccak_f1600_vs_generic", test_keccak_f1600_matches_generic),
    ("sha3_224_empty", test_sha3_224_empty),
    ("sha3_256_empty", test_sha3_256_empty),
    ("sha3_384_empty", test_sha3_384_empty),
    ("sha3_512_empty", test_sha3_512_empty),
    ("sha3_256_abc", test_sha3_256_abc),
    ("sha3_256_multi_block", test_sha3_256_multi_block),
];

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
