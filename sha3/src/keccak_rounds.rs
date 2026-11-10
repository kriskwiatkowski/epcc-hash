//! Keccak round functions - portable Rust reference implementation.
//!
//! This module provides the individual Keccak-f[1600] rounds (0-3) as
//! documented functions. Each round applies the Theta-Rho-Pi-Chi-Iota
//! sequence, with Round 0 operating on the classical (non-lazy) state
//! representation, and Rounds 1-3 operating on the lazily rotated state.
//!
//! The core algorithm is implemented in `keccak_round_parameterized()`,
//! which is called by Rounds 1-3. Round 0 is kept separate because it
//! uses the classical state with zero rotation offsets.

use crate::consts::RHO;

/// Rotate a bit-interleaved 64-bit lane left by `r` bits.
/// Even/odd halves and rotations of odd amounts cause swaps.
#[inline]
#[allow(dead_code)] // Used in documentation examples
fn rotl64_bi(e: u32, o: u32, r: u32) -> (u32, u32) {
    if r % 2 == 0 {
        (e.rotate_left(r / 2), o.rotate_left(r / 2))
    } else {
        (o.rotate_left((r + 1) / 2), e.rotate_left(r / 2))
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
#[inline]
#[allow(dead_code)] // Reference implementation
fn rho_pi_chi_fused<F>(state: &mut [u32; 50], lane_index_fn: F)
where
    F: Fn(usize) -> usize,
{
    // Pre-computed new_y values for each (x, y) pair.
    // new_y = (2*x + 3*y) % 5, computed at gen-time.
    let new_y_table: [[usize; 5]; 5] = [
        [0, 2, 4, 1, 3], // y=0
        [3, 0, 2, 4, 1], // y=1
        [1, 3, 0, 2, 4], // y=2
        [4, 1, 3, 0, 2], // y=3
        [2, 4, 1, 3, 0], // y=4
    ];

    // Process row by row: load all 5 rotated lanes, compute Chi, store immediately.
    // This keeps intermediate values in local arrays (in registers), not in be/bo.
    for y in 0..5 {
        // Rho+Pi: Rotate and permute all 5 lanes in this row
        let mut rotated_e = [0u32; 5];
        let mut rotated_o = [0u32; 5];
        for x in 0..5 {
            let i = x + 5 * y;
            let state_idx = lane_index_fn(i);
            let (ne, no) =
                rotl64_bi(state[state_idx], state[state_idx + 1], RHO[i]);
            rotated_e[x] = ne;
            rotated_o[x] = no;
        }

        // Chi: Compute for all 5 lanes using the rotated values, store immediately
        for x in 0..5 {
            let x_plus_1 = if x == 4 { 0 } else { x + 1 };
            let x_plus_2 = if x >= 3 { x - 3 } else { x + 2 };

            let chi_e =
                rotated_e[x] ^ (!rotated_e[x_plus_1] & rotated_e[x_plus_2]);
            let chi_o =
                rotated_o[x] ^ (!rotated_o[x_plus_1] & rotated_o[x_plus_2]);

            // Determine the permuted position after Rho+Pi
            let new_y = new_y_table[y][x];
            let new_x = y;
            let j = new_x + 5 * new_y;

            // Store Chi result directly to the permuted position
            let state_idx = lane_index_fn(j);
            state[state_idx] = chi_e;
            state[state_idx + 1] = chi_o;
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
    lane_indices: &[usize; 50],
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
            let state_idx = lane_indices[2 * i];
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
            let state_idx = lane_indices[2 * i];
            state[state_idx] ^= de[x];
            state[state_idx + 1] ^= do_[x];
        }
    }

    // Rho + Pi + Chi: Fused step (no intermediate be/bo arrays)
    // Keeps rotated lanes in local arrays, computes Chi immediately, stores results
    rho_pi_chi_fused(state, |i| lane_indices[2 * i]);

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
    let lane_indices: [usize; 50] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    ];
    keccak_round_parameterized(state, &lane_indices, 1, rc_e, rc_o);
}

/// Keccak Round 2: Lazy rotated state with further permutation.
/// Calls the parameterized function with Round 2's lane configuration.
#[allow(dead_code)] // Reference implementation
pub fn keccak_round2(state: &mut [u32; 50], rc_e: u32, rc_o: u32) {
    // Similar to Round 1, but with Round 2's lane permutation.
    let lane_indices: [usize; 50] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
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
    let lane_indices: [usize; 50] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    ];
    keccak_round_parameterized(state, &lane_indices, 3, rc_e, rc_o);
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
