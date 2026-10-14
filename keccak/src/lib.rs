#![cfg_attr(not(feature = "std"), no_std)]

pub mod bitinterleaving;
pub mod keccak_32;
//pub mod keccak_64;
pub mod sha3;

trait Permute {
    fn permute(&mut self);
    fn absorb_block(&mut self, input: &[u8]);
}

#[cfg(target_arch = "arm")]
mod bitinterleaving_armv8m;

pub use bitinterleaving::{conv_from_bi, conv_into_bi};
