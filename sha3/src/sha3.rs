use crate::keccak_32::*;
use crate::keccak_64::*;
use crate::Permute;
use crate::{conv_from_bi, conv_into_bi};
use crate::{KeccakState, KECCAK_STATE_ZERO};
use core::assert;
use core::cmp::Ord;
use core::debug_assert_eq;
use core::default::Default;

const SHA3_DOMAIN: u8 = 0x06;

// Largest rate among SHA3-224/256/384/512, used to size the internal
// block buffer.
const MAX_RATE: usize = 144;

/// Streaming SHA3 hasher. `RATE` is the sponge rate in bytes and
/// `OUTPUT_LEN` the digest length in bytes; use the [`Sha3_224`],
/// [`Sha3_256`], [`Sha3_384`], [`Sha3_512`] aliases rather than naming
/// this type directly.
pub struct Sha3 {
    // Layout depends on `KeccakState` (see `lib.rs`): with the default
    // 64-bit lanes this is the plain Keccak state; with `keccak32bit`,
    // lane `i`'s even/odd halves live at `state[2 * i]`/`state[2 * i + 1]`
    // (see `keccak_32::permute`).
    state: KeccakState,
    buf: [u8; MAX_RATE],
    buf_len: usize,
    rate: usize,
    output_len: usize,
}

impl Sha3 {
    fn new(rate: usize, output_len: usize) -> Self {
        assert!(rate > 0 && rate <= MAX_RATE && rate % 8 == 0);
        assert!(output_len > 0 && output_len <= rate);
        Self {
            state: KECCAK_STATE_ZERO,
            buf: [0u8; MAX_RATE],
            buf_len: 0,
            rate,
            output_len,
        }
    }

    pub fn Sha3_256() -> Self {
        Self::new(136, 32)
    }

    pub fn Sha3_384() -> Self {
        Self::new(104, 48)
    }

    pub fn Sha3_512() -> Self {
        Self::new(72, 64)
    }

    pub fn update(&mut self, mut data: &[u8]) {
        if self.buf_len > 0 {
            let take = (self.rate - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take]
                .copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
            if self.buf_len == self.rate {
                self.absorb_buffered_block();
                self.buf_len = 0;
            }
        }

        while data.len() >= self.rate {
            let (block, rest) = data.split_at(self.rate);
            self.absorb_block(block);
            data = rest;
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
        }
    }

    fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), self.rate);
        self.state.absorb_block(block);
        self.state.permute();
    }

    fn absorb_buffered_block(&mut self) {
        self.state.absorb_block(&self.buf[..self.rate]);
        self.state.permute();
    }

    pub fn finalize(mut self, out: &mut [u8]) -> &[u8] {
        // Pad the remaining bytes in the buffer and absorb the final block.
        for b in &mut self.buf[self.buf_len..self.rate] {
            *b = 0;
        }
        self.buf[self.buf_len] ^= SHA3_DOMAIN;
        self.buf[self.rate - 1] ^= 0x80;
        self.absorb_buffered_block();

        self.state.squeeze_block(out);
        out
    }
}

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha3::Sha3_256();
    h.update(data);
    let mut out = [0u8; 32];
    h.finalize(&mut out);
    out
}

pub fn sha3_512(data: &[u8]) -> [u8; 64] {
    let mut h = Sha3::Sha3_512();
    h.update(data);
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}
