#![cfg_attr(not(feature = "std"), no_std)]

pub mod bitinterleaving;
pub mod permutation;
pub mod sha3;

#[cfg(target_arch = "arm")]
mod bitinterleaving_armv8m;

#[cfg(all(target_arch = "riscv32", target_feature = "zbkb"))]
mod bitinterleaving_rv32;

// `rori` and `andn` are in both Zbb and Zbkb, so either is enough for
// the permutation; only the conversions above need Zbkb specifically.
#[cfg(all(
    target_arch = "riscv32",
    any(target_feature = "zbb", target_feature = "zbkb")
))]
mod permutation_rv32;

pub use bitinterleaving::{conv_from_bi, conv_into_bi};
pub use permutation::{keccak_f1600, keccak_f1600_generic};
pub use sha3::{
    sha3_224, sha3_256, sha3_384, sha3_512, Sha3_224, Sha3_256, Sha3_384,
    Sha3_512,
};
