//! Keccak-f\[1600\] permutation on the bit-interleaved state.
//!
//! Each 64-bit lane is kept as a pair of 32-bit halves: the even-indexed
//! bits of the lane packed into one `u32`, the odd-indexed bits packed
//! into the other (see [`crate::conv_into_bi`]/[`crate::conv_from_bi`]).
//! This lets every step of the round function run with only 32-bit
//! operations, which is what lets the ARMv7-M assembly in
//! `origin/keccak-nrv/keccakf1600.S` (`keccak_f1600_permute`) avoid
//! emulating 64-bit rotates. This module computes the same permutation,
//! following the same lane layout, rotation offsets and round constants
//! (the latter split into even/odd halves exactly as in
//! `KeccakF1600_StatePermute_RoundConstantsWithTerminator`), but is plain
//! portable Rust rather than hand-written Cortex-M assembly.
//!
//! [`crate::permutation_64`] is the straightforward 64-bit-lane reference
//! implementation this one is checked against.
//!
//! Lane `(x, y)` lives at `state[2 * (x + 5 * y)]` (even half) and
//! `state[2 * (x + 5 * y) + 1]` (odd half), matching the assembly's
//! `Aba0`/`Aba1` .. `Asu0`/`Asu1` offsets.

use crate::Permute;

const ROUNDS: usize = 24;
use crate::{conv_from_bi, conv_into_bi};

// Same offsets as `crate::permutation_64::RHO`, indexed by `x + 5 * y`.
const RHO: [u32; 25] = [
    0, 1, 62, 28, 27, //
    36, 44, 6, 55, 20, //
    3, 10, 43, 25, 39, //
    41, 45, 15, 21, 8, //
    18, 2, 61, 56, 14,
];

// Round constants from `KeccakF1600_StatePermute_RoundConstantsWithTerminator`
// in the assembly, split into their even-bit and odd-bit halves (so that
// `RCE[r]` combined with `RCO[r]` the way [`crate::conv_from_bi`] combines
// a lane's halves equals `crate::permutation_64::RC[r]`).
const RCE: [u32; ROUNDS] = [
    0x0000_0001,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0001,
    0x0000_0001,
    0x0000_0001,
    0x0000_0001,
    0x0000_0000,
    0x0000_0000,
    0x0000_0001,
    0x0000_0000,
    0x0000_0001,
    0x0000_0001,
    0x0000_0001,
    0x0000_0001,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0000,
    0x0000_0001,
    0x0000_0000,
    0x0000_0001,
    0x0000_0000,
];

const RCO: [u32; ROUNDS] = [
    0x0000_0000,
    0x0000_0089,
    0x8000_008b,
    0x8000_8080,
    0x0000_008b,
    0x0000_8000,
    0x8000_8088,
    0x8000_0082,
    0x0000_000b,
    0x0000_000a,
    0x0000_8082,
    0x0000_8003,
    0x0000_808b,
    0x8000_000b,
    0x8000_008a,
    0x8000_0081,
    0x8000_0081,
    0x8000_0008,
    0x0000_0083,
    0x8000_8003,
    0x8000_8088,
    0x8000_0088,
    0x0000_8000,
    0x8000_8082,
];

/// Rotate a bit-interleaved 64-bit lane (given as its even/odd 32-bit
/// halves) left by `r` bits, returning the new (even, odd) halves.
///
/// A rotation by an even amount keeps even bits even and odd bits odd, so
/// it is just a 32-bit rotation of each half by `r / 2`. A rotation by an
/// odd amount moves every even bit to an odd position and vice versa, so
/// the halves swap (with a +1 adjustment on the shift for the half that
/// picks up the extra bit).
fn rotl64_bi(e: u32, o: u32, r: u32) -> (u32, u32) {
    if r.is_multiple_of(2) {
        (e.rotate_left(r / 2), o.rotate_left(r / 2))
    } else {
        (o.rotate_left(r.div_ceil(2)), e.rotate_left((r - 1) / 2))
    }
}

impl Permute for [u32; 50] {
    /// Apply the Keccak-f\[1600\] permutation to the bit-interleaved `state`
    /// (24 rounds of Theta, Rho, Pi, Chi, Iota).
    fn permute(&mut self) {
        let mut state = self;
        for round in 0..ROUNDS {
            // Theta
            let mut ce = [0u32; 5];
            let mut co = [0u32; 5];
            for x in 0..5 {
                let mut e = 0u32;
                let mut o = 0u32;
                for y in 0..5 {
                    let i = x + 5 * y;
                    e ^= state[2 * i];
                    o ^= state[2 * i + 1];
                }
                ce[x] = e;
                co[x] = o;
            }

            // D[x] = C[x - 1] xor rotl64(C[x + 1], 1); rotl64(_, 1) is odd, so
            // it swaps the even/odd halves (see rotl64_bi below).
            let mut de = [0u32; 5];
            let mut do_ = [0u32; 5];
            for x in 0..5 {
                let (re, ro) = rotl64_bi(ce[(x + 1) % 5], co[(x + 1) % 5], 1);
                de[x] = ce[(x + 4) % 5] ^ re;
                do_[x] = co[(x + 4) % 5] ^ ro;
            }
            for y in 0..5 {
                for x in 0..5 {
                    let i = x + 5 * y;
                    state[2 * i] ^= de[x];
                    state[2 * i + 1] ^= do_[x];
                }
            }

            // Rho + Pi
            let mut be = [0u32; 25];
            let mut bo = [0u32; 25];
            for y in 0..5 {
                for x in 0..5 {
                    let i = x + 5 * y;
                    let (ne, no) =
                        rotl64_bi(state[2 * i], state[2 * i + 1], RHO[i]);
                    let new_x = y;
                    let new_y = (2 * x + 3 * y) % 5;
                    let j = new_x + 5 * new_y;
                    be[j] = ne;
                    bo[j] = no;
                }
            }

            // Chi
            for y in 0..5 {
                for x in 0..5 {
                    let i = x + 5 * y;
                    let i1 = (x + 1) % 5 + 5 * y;
                    let i2 = (x + 2) % 5 + 5 * y;
                    state[2 * i] = be[i] ^ (!be[i1] & be[i2]);
                    state[2 * i + 1] = bo[i] ^ (!bo[i1] & bo[i2]);
                }
            }

            // Iota
            state[0] ^= RCE[round];
            state[1] ^= RCO[round];
        }
    }

    fn absorb_block(&mut self, input: &[u8]) {
        for (i, chunk) in input.chunks_exact(8).enumerate() {
            let lo = u32::from_le_bytes(chunk[..4].try_into().unwrap());
            let hi = u32::from_le_bytes(chunk[4..].try_into().unwrap());
            let (e, o) = conv_into_bi(lo, hi);
            self[2 * i + 0] ^= e;
            self[2 * i + 1] ^= o;
        }
    }
}
