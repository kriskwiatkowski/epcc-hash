//! `no_std` FIPS-202 implementation.

#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "testing")]
pub mod bitinterleaving;
#[cfg(not(feature = "testing"))]
mod bitinterleaving;
mod consts;
mod keccak_32;
mod keccak_64;
pub mod sha3;

/// Apply the keccak permutation to `state` (24 rounds of
/// Theta, Rho, Pi, Chi, Iota).
trait Permute {
    /// Apply the keccak permutation to the state.
    fn permute(&mut self);
    /// Absorb a block of input into the state.
    fn absorb_block(&mut self, input: &[u8]);
    /// Squeeze a block of output from the state.
    fn squeeze_block(&mut self, output: &mut [u8]);
}

/// The Keccak-f\[1600\] state representation `Sha3` is built on, chosen at
/// compile time via the `keccak32bit` feature rather than by target: the
/// 32-bit, bit-interleaved layout (`keccak_32`) or the plain 64-bit-lane
/// one (`keccak_64`). Both implement `Permute`, so this is the only switch
/// needed to retarget `Sha3`.
#[cfg(feature = "keccak32bit")]
pub(crate) type KeccakState = [u32; 50];
#[cfg(not(feature = "keccak32bit"))]
pub(crate) type KeccakState = [u64; 25];

// `[T; N]: Default` only covers N <= 32 in std, and 50 doesn't qualify, so
// the zero state is spelled out per backend instead.
#[cfg(feature = "keccak32bit")]
pub(crate) const KECCAK_STATE_ZERO: KeccakState = [0u32; 50];
#[cfg(not(feature = "keccak32bit"))]
pub(crate) const KECCAK_STATE_ZERO: KeccakState = [0u64; 25];

#[cfg(target_arch = "arm")]
mod bitinterleaving_armv8m;

pub use bitinterleaving::{conv_from_bi, conv_into_bi};
