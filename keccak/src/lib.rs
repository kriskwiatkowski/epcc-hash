#![cfg_attr(not(feature = "std"), no_std)]

pub mod bitinterleaving;

#[cfg(target_arch = "arm")]
mod bitinterleaving_armv8m;

pub use bitinterleaving::{conv_from_bi, conv_into_bi};
