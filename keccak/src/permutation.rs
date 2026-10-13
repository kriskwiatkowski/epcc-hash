//! Keccak-f[1600] permutation, computed entirely in the bit-interleaved
//! domain.
//!
//! Every lane is a 64-bit value split into an even-bit word and an
//! odd-bit word (see [`crate::conv_into_bi`] / [`crate::conv_from_bi`]).
//! In that representation a 64-bit rotate by `n` bits reduces to one or
//! two native 32-bit rotates (see [`rotl64_bi`]), and every other step
//! (theta, pi, chi, iota) is a plain bitwise operation or lane shuffle
//! that is unaffected by the interleaving. So the whole permutation runs
//! without ever touching a 64-bit value: lanes are converted in once on
//! entry and back out once on exit.

use crate::{conv_from_bi, conv_into_bi};

const ROUNDS: usize = 24;

// Rotation offsets and the rho/pi lane-traversal order, indexed by round
// step (not by (x, y)); combining them in one pass is the standard
// compact formulation of Keccak-f's rho + pi steps.
const ROTC: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18,
    39, 61, 20, 44,
];
const PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14,
    22, 9, 6, 1,
];

// Round constants, pre-converted to bit-interleaved (even, odd) form so
// iota is a plain XOR against lane (0, 0).
const RC_BI: [(u32, u32); 24] = [
    (0x00000001, 0x00000000),
    (0x00000000, 0x00000089),
    (0x00000000, 0x8000008b),
    (0x00000000, 0x80008080),
    (0x00000001, 0x0000008b),
    (0x00000001, 0x00008000),
    (0x00000001, 0x80008088),
    (0x00000001, 0x80000082),
    (0x00000000, 0x0000000b),
    (0x00000000, 0x0000000a),
    (0x00000001, 0x00008082),
    (0x00000000, 0x00008003),
    (0x00000001, 0x0000808b),
    (0x00000001, 0x8000000b),
    (0x00000001, 0x8000008a),
    (0x00000001, 0x80000081),
    (0x00000000, 0x80000081),
    (0x00000000, 0x80000008),
    (0x00000000, 0x00000083),
    (0x00000000, 0x80008003),
    (0x00000001, 0x80008088),
    (0x00000000, 0x80000088),
    (0x00000001, 0x00008000),
    (0x00000000, 0x80008082),
];

// Rotate-left by `n` bits on a 64-bit lane held as (even, odd) 32-bit
// halves. Derived from splitting ROTL64(a, n)_i = a[(i - n) mod 64]
// into its even/odd index residues: an even `n` rotates each half by
// n/2 independently; an odd `n` additionally swaps the two halves.
#[inline]
fn rotl64_bi(e: u32, o: u32, n: u32) -> (u32, u32) {
    if n % 2 == 0 {
        let m = n / 2;
        (e.rotate_left(m), o.rotate_left(m))
    } else {
        let m = n / 2;
        (o.rotate_left(m + 1), e.rotate_left(m))
    }
}

/// Apply the Keccak-f[1600] permutation to `state`, given as 25 lanes in
/// natural (non-interleaved) form, lane `x + 5*y` at index `x + 5*y`.
///
/// Dispatches to a hand-written implementation where one exists for the
/// target; [`keccak_f1600_generic`] is always available for comparison.
pub fn keccak_f1600(state: &mut [u64; 25]) {
    cfg_if::cfg_if! {
        if #[cfg(all(
            target_arch = "riscv32",
            any(target_feature = "zbb", target_feature = "zbkb")
        ))] {
            keccak_f1600_rv32(state)
        } else {
            keccak_f1600_generic(state)
        }
    }
}

/// Portable Keccak-f[1600]. Kept public so tests can check an
/// architecture-specific implementation against it on its own hardware.
pub fn keccak_f1600_generic(state: &mut [u64; 25]) {
    let mut even = [0u32; 25];
    let mut odd = [0u32; 25];
    for i in 0..25 {
        let (e, o) = conv_into_bi(state[i] as u32, (state[i] >> 32) as u32);
        even[i] = e;
        odd[i] = o;
    }

    for round in 0..ROUNDS {
        // Theta
        let mut ce = [0u32; 5];
        let mut co = [0u32; 5];
        for x in 0..5 {
            ce[x] = even[x]
                ^ even[x + 5]
                ^ even[x + 10]
                ^ even[x + 15]
                ^ even[x + 20];
            co[x] =
                odd[x] ^ odd[x + 5] ^ odd[x + 10] ^ odd[x + 15] ^ odd[x + 20];
        }
        for x in 0..5 {
            let (re, ro) = rotl64_bi(ce[(x + 1) % 5], co[(x + 1) % 5], 1);
            let de = ce[(x + 4) % 5] ^ re;
            let do_ = co[(x + 4) % 5] ^ ro;
            for y in 0..5 {
                even[x + 5 * y] ^= de;
                odd[x + 5 * y] ^= do_;
            }
        }

        // Rho + Pi
        let mut te = even[1];
        let mut to = odd[1];
        for i in 0..ROUNDS {
            let j = PILN[i];
            let be = even[j];
            let bo = odd[j];
            let (re, ro) = rotl64_bi(te, to, ROTC[i]);
            even[j] = re;
            odd[j] = ro;
            te = be;
            to = bo;
        }

        // Chi
        for row in 0..5 {
            let base = row * 5;
            let be = [
                even[base],
                even[base + 1],
                even[base + 2],
                even[base + 3],
                even[base + 4],
            ];
            let bo = [
                odd[base],
                odd[base + 1],
                odd[base + 2],
                odd[base + 3],
                odd[base + 4],
            ];
            for x in 0..5 {
                even[base + x] = be[x] ^ (!be[(x + 1) % 5] & be[(x + 2) % 5]);
                odd[base + x] = bo[x] ^ (!bo[(x + 1) % 5] & bo[(x + 2) % 5]);
            }
        }

        // Iota
        let (rce, rco) = RC_BI[round];
        even[0] ^= rce;
        odd[0] ^= rco;
    }

    for i in 0..25 {
        let (lo, hi) = conv_from_bi(even[i], odd[i]);
        state[i] = (lo as u64) | ((hi as u64) << 32);
    }
}

/// Keccak-f[1600] via the RV32 Zbb assembly permutation.
///
/// The assembly reads lane `i` as two 32-bit words at byte offsets `8*i`
/// and `8*i + 4` -- the even half first. On a little-endian target that
/// is exactly the layout of a `u64` holding `even | (odd << 32)`, so the
/// lanes can be converted in place and handed over without a copy.
#[cfg(all(
    target_arch = "riscv32",
    any(target_feature = "zbb", target_feature = "zbkb")
))]
fn keccak_f1600_rv32(state: &mut [u64; 25]) {
    const _: () = assert!(cfg!(target_endian = "little"));

    for lane in state.iter_mut() {
        let (e, o) = conv_into_bi(*lane as u32, (*lane >> 32) as u32);
        *lane = (e as u64) | ((o as u64) << 32);
    }

    // SAFETY: the callee reads and writes exactly the 50 words backing
    // these 25 lanes, and needs no alignment beyond a `u64`'s.
    unsafe {
        crate::permutation_rv32::keccak_f1600_bi_rv32(
            state.as_mut_ptr().cast(),
        );
    }

    for lane in state.iter_mut() {
        let (lo, hi) = conv_from_bi(*lane as u32, (*lane >> 32) as u32);
        *lane = (lo as u64) | ((hi as u64) << 32);
    }
}
