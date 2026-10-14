//! Keccak constants shared by both `Permute` backends
//! ([`crate::keccak_32`], [`crate::keccak_64`]).

// Number of rounds in the Keccak-f[1600] permutation.
pub(crate) const ROUNDS: usize = 24;

// Round constants for the Keccak-f[1600] permutation.
pub(crate) const RHO: [u32; 25] = [
    0, 1, 62, 28, 27, //
    36, 44, 6, 55, 20, //
    3, 10, 43, 25, 39, //
    41, 45, 15, 21, 8, //
    18, 2, 61, 56, 14,
];

// SHA3 domain separation constant for padding.
pub(crate) const SHA3_DOMAIN: u8 = 0x06;
// SHAKE domain separation constant for padding.
const SHAKE_DOMAIN: u8 = 0x1F;

// Largest rate among SHA3-224/256/384/512, used to size the internal
// block buffer.
pub(crate) const MAX_RATE: usize = 144;
