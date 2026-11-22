//! Internal `Permute` backend: for 32-bit keccak on the bit-interleaved
//! state.
//!
//! Each 64-bit lane is kept as a pair of 32-bit halves: the even-indexed
//! bits of the lane packed into one `u32`, the odd-indexed bits packed
//! into the other (see [`crate::conv_into_bi`]/[`crate::conv_from_bi`]).
//! This lets every step of the round function run with only 32-bit
//! operations, which avoids emulating 64-bit rotates. This module
//! computes the same permutation, following the same lane layout,
//! rotation offsets and round constants, but is plain
//! portable Rust rather than hand-written Cortex-M assembly.

use crate::consts::{RHO, ROUNDS};
use crate::Permute;
use crate::{conv_from_bi, conv_into_bi};
use core::convert::TryInto;
use core::iter::Iterator;

#[cfg(all(target_arch = "arm", not(feature = "no_asm")))]
use crate::keccak_permute_armv8m::epcc_keccak_permute_asm;

// Round constants split into their even-bit and odd-bit halves (so that
// `RC[r].even` combined with `RC[r].odd` the way [`crate::conv_from_bi`]
// combines a lane's halves equals `crate::permutation_64::RC[r]`).
struct RcT {
    even: u32,
    odd: u32,
}

const RC: [RcT; ROUNDS] = [
    RcT { even: 0x0000_0001, odd: 0x0000_0000 },
    RcT { even: 0x0000_0000, odd: 0x0000_0089 },
    RcT { even: 0x0000_0000, odd: 0x8000_008b },
    RcT { even: 0x0000_0000, odd: 0x8000_8080 },
    RcT { even: 0x0000_0001, odd: 0x0000_008b },
    RcT { even: 0x0000_0001, odd: 0x0000_8000 },
    RcT { even: 0x0000_0001, odd: 0x8000_8088 },
    RcT { even: 0x0000_0001, odd: 0x8000_0082 },
    RcT { even: 0x0000_0000, odd: 0x0000_000b },
    RcT { even: 0x0000_0000, odd: 0x0000_000a },
    RcT { even: 0x0000_0001, odd: 0x0000_8082 },
    RcT { even: 0x0000_0000, odd: 0x0000_8003 },
    RcT { even: 0x0000_0001, odd: 0x0000_808b },
    RcT { even: 0x0000_0001, odd: 0x8000_000b },
    RcT { even: 0x0000_0001, odd: 0x8000_008a },
    RcT { even: 0x0000_0001, odd: 0x8000_0081 },
    RcT { even: 0x0000_0000, odd: 0x8000_0081 },
    RcT { even: 0x0000_0000, odd: 0x8000_0008 },
    RcT { even: 0x0000_0000, odd: 0x0000_0083 },
    RcT { even: 0x0000_0000, odd: 0x8000_8003 },
    RcT { even: 0x0000_0001, odd: 0x8000_8088 },
    RcT { even: 0x0000_0000, odd: 0x8000_0088 },
    RcT { even: 0x0000_0001, odd: 0x0000_8000 },
    RcT { even: 0x0000_0000, odd: 0x8000_8082 },
];

/// Rotate a bit-interleaved 64-bit lane left by `r` bits.
/// Even/odd halves and rotations of odd amounts cause swaps.
#[inline]
#[allow(dead_code)] // Used in documentation examples
fn rotl64_bi(e: u32, o: u32, r: u32) -> (u32, u32) {
    if r % 2 == 0 {
        // todo & 1
        (e.rotate_left(r >> 1), o.rotate_left(r >> 1))
    } else {
        (o.rotate_left((r + 1) >> 1), e.rotate_left(r >> 1))
    }
}

/// Rotate a bit-interleaved 64-bit lane left by `r` bits.
/// Even/odd halves and rotations of odd amounts cause swaps.
#[inline]
#[allow(dead_code)] // Used in documentation examples
fn rotl64(w: u32, r: u32, eo: usize) -> u32 {
    if r & 1 == 0 {
        // Rotate is even -> odd stays odd, even stays even
        w.rotate_left(r >> 1)
    } else {
        // Rotate is odd -> odd becomes even, even becomes odd
        if eo == 0 {
            w.rotate_left(r >> 1)
        } else {
            w.rotate_left((r + 1) >> 1)
        }
    }
}

/// XOR five lanes and return the even/odd halves of the result.
/// Used in the Theta step.
#[inline]
#[allow(dead_code)] // Helper function for reference
fn xor5_bi(
    e0: u32,
    o0: u32,
    e1: u32,
    o1: u32,
    e2: u32,
    o2: u32,
    e3: u32,
    o3: u32,
    e4: u32,
    o4: u32,
) -> (u32, u32) {
    (e0 ^ e1 ^ e2 ^ e3 ^ e4, o0 ^ o1 ^ o2 ^ o3 ^ o4)
}

/// Compute D[x] = C[x-1] XOR rotl(C[x+1], 1) for the Theta step.
/// Returns (d_even, d_odd).
#[inline]
#[allow(dead_code)] // Helper function for reference
fn theta_d_bi(c_e: &[u32; 5], c_o: &[u32; 5], x: usize) -> (u32, u32) {
    // C[x+1] rotated left by 1 (odd rotation swaps halves)
    let x_plus_1 = if x == 4 { 0 } else { x + 1 };
    let x_minus_1 = if x == 0 { 4 } else { x - 1 };
    let (re, ro) = rotl64_bi(c_e[x_plus_1], c_o[x_plus_1], 1);
    // D[x] = C[x-1] XOR rotl(C[x+1], 1)
    (c_e[x_minus_1] ^ re, c_o[x_minus_1] ^ ro)
}

/// Rho + Pi + Chi fused step - keeps rotated lanes in local arrays instead of global be/bo.
///
/// This mirrors the assembly's approach: process row by row, loading rotated lanes,
/// computing Chi immediately without intermediate array storage. Keeps hot data in
/// registers to minimize cache misses and enable instruction-level parallelism.
/// TODO: Next step is to make this function work sequentially - first with odd then even halves of the lanes.
#[inline]
#[allow(dead_code)] // Reference implementation
fn rho_pi_chi_fused<F>(state: &mut [u32; 50], lane_index_fn: F)
where
    F: Fn(usize) -> usize,
{
    // Source lane index for each output lane: PI_SRC[y_out][x_out] is the
    // index i = x + 5y of the input lane A[x, y] that Pi moves to
    // B[x_out, y_out], i.e. (x_out + 3 * y_out) % 5 + 5 * x_out.
    const PI_SRC: [[usize; 5]; 5] = [
        [0, 6, 12, 18, 24], // y_out=0
        [3, 9, 10, 16, 22], // y_out=1
        [1, 7, 13, 19, 20], // y_out=2
        [4, 5, 11, 17, 23], // y_out=3
        [2, 8, 14, 15, 21], // y_out=4
    ];

    let mut rotated_e = [0u32; 5];
    let mut rotated_o = [0u32; 5];
    for y_out in 0..5 {
        // Rho+Pi: gather the 5 rotated lanes forming output row `y_out`.
        for x_out in 0..5 {
            let i = PI_SRC[y_out][x_out];
            let state_idx = lane_index_fn(i);
            let ne = rotl64(state[state_idx + 0], RHO[i], 0);
            let no = rotl64(state[state_idx + 1], RHO[i], 1);
            // If value of RHO[i] is odd, then even->odd and odd->even.
            if RHO[i] & 1 == 0 {
                rotated_e[x_out] = ne;
                rotated_o[x_out] = no;
            } else {
                rotated_e[x_out] = no;
                rotated_o[x_out] = ne;
            }
        }

        // Chi along the output row.
        for x in 0..5 {
            let x_plus_1 = if x == 4 { 0 } else { x + 1 };
            let x_plus_2 = if x >= 3 { x - 3 } else { x + 2 };
            // TODO: it works either by chance or I don't fully get it.
            // 1. why % 5
            // 2. how comes it PI_SRC is identity after 4 rounds (must be related)
            let i = lane_index_fn(PI_SRC[y_out][(x + y_out) % 5]);

            state[i + 0] =
                rotated_e[x] ^ (!rotated_e[x_plus_1] & rotated_e[x_plus_2]);
            state[i + 1] =
                rotated_o[x] ^ (!rotated_o[x_plus_1] & rotated_o[x_plus_2]);
        }
    }
}

/// Generic Keccak round with parameterized lane arguments.
/// Used by Rounds 1-3 with their respective lane orderings and rotations.
///
/// This implements the core 38-instruction sequence:
/// - Theta step with pre-computed lane values
/// - Rho + Pi transformation
/// - Chi step
/// - Iota step
///
/// Note: This version unrolls the Rho+Pi and Chi steps completely (all 25 iterations),
/// matching the codegen approach where (2*x + 3*y) % 5 is pre-computed at gen-time.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round_parameterized(
    state: &mut [u32; 50],
    // 50 lane variable arguments: Aba0, Aba1, Abe0, Abe1, ..., Asu0, Asu1
    // Since Rust doesn't have tuple unpacking in function params easily,
    // we index into the state array directly using pre-mapped lane indices.
    lane_indices: &[usize; 25],
    _round: usize, // Round number for reference/documentation
    rc_e: u32,
    rc_o: u32,
) {
    // In the Rust implementation, we work directly with the state array.
    // The lane_indices array maps the logical lanes from the Python code
    // to actual state array indices. For most applications, this would be
    // [0, 1, 2, 3, ..., 99] (sequential), but it allows flexibility for
    // different permutation strategies.

    // Theta: Compute C[x] = XOR of column x
    let mut ce = [0u32; 5];
    let mut co = [0u32; 5];
    for x in 0..5 {
        let mut e = 0u32;
        let mut o = 0u32;
        for y in 0..5 {
            let i = x + 5 * y;
            let state_idx = 2 * lane_indices[i];
            e ^= state[state_idx];
            o ^= state[state_idx + 1];
        }
        ce[x] = e;
        co[x] = o;
    }

    // D[x] = C[x-1] XOR rotl(C[x+1], 1)
    let mut de = [0u32; 5];
    let mut do_ = [0u32; 5];
    for x in 0..5 {
        (de[x], do_[x]) = theta_d_bi(&ce, &co, x);
    }

    // Apply D to all lanes
    for y in 0..5 {
        for x in 0..5 {
            let i = x + 5 * y;
            let state_idx = 2 * lane_indices[i];
            state[state_idx] ^= de[x];
            state[state_idx + 1] ^= do_[x];
        }
    }

    // Rho + Pi + Chi: Fused step (no intermediate be/bo arrays)
    // Keeps rotated lanes in local arrays, computes Chi immediately, stores results
    rho_pi_chi_fused(state, |i| 2 * lane_indices[i]);

    // Iota (add round constant to state[0])
    state[0] ^= rc_e;
    state[1] ^= rc_o;
}

/// Keccak Round 0: Classical state representation.
/// Operates on the fresh, unrotated state with all rotation offsets at zero.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round0(state: &mut [u32; 50], rc_e: u32, rc_o: u32) {
    // Theta: Compute C[x] = XOR of column x
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

    // D[x] = C[x-1] XOR rotl(C[x+1], 1)
    let mut de = [0u32; 5];
    let mut do_ = [0u32; 5];
    for x in 0..5 {
        (de[x], do_[x]) = theta_d_bi(&ce, &co, x);
    }

    // Apply D to all lanes
    for y in 0..5 {
        for x in 0..5 {
            let i = x + 5 * y;
            state[2 * i + 0] ^= de[x];
            state[2 * i + 1] ^= do_[x];
        }
    }

    // Rho + Pi + Chi: Fused step (no intermediate be/bo arrays)
    // Keeps rotated lanes in local arrays, computes Chi immediately, stores results
    rho_pi_chi_fused(state, |i| 2 * i);

    // Iota
    state[0] ^= rc_e;
    state[1] ^= rc_o;
}

/// Keccak Round 1: Lazy rotated state with permuted lane ordering.
/// Calls the parameterized function with Round 1's lane configuration.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round1(state: &mut [u32; 50], rc_e: u32, rc_o: u32) {
    // For Round 1 with lazy rotation, we would pass a lane_indices array
    // that reflects the permutation and rotation state. In practice, for
    // a true lazy-rotation variant, this would be generated by the code
    // generator or computed based on the Keccak permutation schedule.
    // For now, use identity mapping (sequential).
    let lane_indices: [usize; 25] = [
        0, 6, 12, 18, 24, 9, 10, 16, 22, 3, 13, 19, 20, 1, 7, 17, 23, 4, 5, 11,
        21, 2, 8, 14, 15,
    ];
    keccak_round_parameterized(state, &lane_indices, 1, rc_e, rc_o);
}

/// Keccak Round 2: Lazy rotated state with further permutation.
/// Calls the parameterized function with Round 2's lane configuration.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round2(state: &mut [u32; 50], rc_e: u32, rc_o: u32) {
    // Similar to Round 1, but with Round 2's lane permutation.
    let lane_indices: [usize; 25] = [
        0, 10, 20, 5, 15, 3, 13, 23, 8, 18, 1, 11, 21, 6, 16, 4, 14, 24, 9, 19,
        2, 12, 22, 7, 17,
    ];
    keccak_round_parameterized(state, &lane_indices, 2, rc_e, rc_o);
}

/// Keccak Round 3: Final lazy rotated round.
/// Calls the parameterized function with Round 3's lane configuration.
///
/// Note: The Python assembly version ends with `STR_W(r1, [r0, Aba1])`, an ARM Thumb-2
/// store instruction that writes a computed value back to memory. In Rust, we achieve
/// the same result through in-place mutation of the `state` array. The parameterized
/// function modifies state directly, eliminating the need for an explicit final store
/// operation. This is functionally equivalent but uses language-idiomatic approaches
/// rather than explicit memory operations.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round3(state: &mut [u32; 50], rc_e: u32, rc_o: u32) {
    // Final round with Round 3's lane permutation.
    let lane_indices: [usize; 25] = [
        0, 13, 21, 9, 17, 18, 1, 14, 22, 5, 6, 19, 2, 10, 23, 24, 7, 15, 3, 11,
        12, 20, 8, 16, 4,
    ];
    keccak_round_parameterized(state, &lane_indices, 3, rc_e, rc_o);
}

/// Apply the Keccak-f\[1600\] permutation to the bit-interleaved `state`
/// (24 rounds of Theta, Rho, Pi, Chi, Iota).
#[cfg(not(feature = "keccak32-unrolled"))]
fn permute_small(state: &mut [u32; 50]) {
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
        state[0] ^= RC[round].even;
        state[1] ^= RC[round].odd;
    }
}

#[cfg(feature = "keccak32-unrolled")]
fn permute_unrolled(state: &mut [u32; 50]) {
    for i in 0..(ROUNDS / 4) {
        keccak_round0(state, RC[4 * i + 0].even, RC[4 * i + 0].odd);
        keccak_round1(state, RC[4 * i + 1].even, RC[4 * i + 1].odd);
        keccak_round2(state, RC[4 * i + 2].even, RC[4 * i + 2].odd);
        keccak_round3(state, RC[4 * i + 3].even, RC[4 * i + 3].odd);
    }
}

impl Permute for [u32; 50] {
    fn permute(&mut self) {
        cfg_if::cfg_if! {
            if #[cfg(all(target_arch = "arm", not(feature = "no_asm")))] {
                unsafe {
                    epcc_keccak_permute_asm(self.as_mut_ptr());
                }
            } else if #[cfg(feature = "keccak32-unrolled")] {
                permute_unrolled(self);
            } else {
                permute_small(self);
            }
        }
    }

    fn absorb_block(&mut self, input: &[u8]) {
        for (i, chunk) in input.chunks_exact(8).enumerate() {
            let lo_bytes: [u8; 4] = chunk[..4].try_into().unwrap();
            let hi_bytes: [u8; 4] = chunk[4..].try_into().unwrap();
            let lo = u32::from_le_bytes(lo_bytes);
            let hi = u32::from_le_bytes(hi_bytes);
            let (e, o) = conv_into_bi(lo, hi);
            self[2 * i + 0] ^= e;
            self[2 * i + 1] ^= o;
        }
    }

    fn squeeze_block(&mut self, output: &mut [u8]) {
        for (i, chunk) in output.chunks_exact_mut(8).enumerate() {
            let (lo, hi) = conv_from_bi(self[2 * i], self[2 * i + 1]);
            chunk[..4].copy_from_slice(&lo.to_le_bytes());
            chunk[4..].copy_from_slice(&hi.to_le_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotl64_bi_even() {
        // Even rotation should rotate each half independently
        let (e, o) = rotl64_bi(0x1234_5678, 0x9abc_def0, 4);
        assert_eq!(e, 0x1234_5678u32.rotate_left(2));
        assert_eq!(o, 0x9abc_def0u32.rotate_left(2));
    }

    #[test]
    fn test_rotl64_bi_odd() {
        // Odd rotation should swap halves
        let (e, o) = rotl64_bi(0x1234_5678, 0x9abc_def0, 1);
        // After odd rotation, halves swap
        assert_eq!(e, 0x9abc_def0u32.rotate_left(1));
        assert_eq!(o, 0x1234_5678u32.rotate_left(0));
    }

    #[test]
    fn test_xor5_bi() {
        let (e, o) = xor5_bi(
            0x1111_1111,
            0x2222_2222,
            0x3333_3333,
            0x4444_4444,
            0x5555_5555,
            0x6666_6666,
            0x7777_7777,
            0x8888_8888,
            0x9999_9999,
            0xaaaa_aaaa,
        );
        assert_eq!(
            e,
            0x1111_1111 ^ 0x3333_3333 ^ 0x5555_5555 ^ 0x7777_7777 ^ 0x9999_9999
        );
        assert_eq!(
            o,
            0x2222_2222 ^ 0x4444_4444 ^ 0x6666_6666 ^ 0x8888_8888 ^ 0xaaaa_aaaa
        );
    }
}
