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
pub struct Sha3<const RATE: usize, const OUTPUT_LEN: usize> {
    // Layout depends on `KeccakState` (see `lib.rs`): with the default
    // 64-bit lanes this is the plain Keccak state; with `keccak32bit`,
    // lane `i`'s even/odd halves live at `state[2 * i]`/`state[2 * i + 1]`
    // (see `keccak_32::permute`).
    state: KeccakState,
    buf: [u8; MAX_RATE],
    buf_len: usize,
}

impl<const RATE: usize, const OUTPUT_LEN: usize> Sha3<RATE, OUTPUT_LEN> {
    pub fn new() -> Self {
        const {
            assert!(RATE > 0 && RATE <= MAX_RATE && RATE % 8 == 0);
            assert!(OUTPUT_LEN > 0 && OUTPUT_LEN <= RATE);
        }
        Self { state: KECCAK_STATE_ZERO, buf: [0u8; MAX_RATE], buf_len: 0 }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        if self.buf_len > 0 {
            let take = (RATE - self.buf_len).min(data.len());
            self.buf[self.buf_len..self.buf_len + take]
                .copy_from_slice(&data[..take]);
            self.buf_len += take;
            data = &data[take..];
            if self.buf_len == RATE {
                self.absorb_buffered_block();
                self.buf_len = 0;
            }
        }

        while data.len() >= RATE {
            let (block, rest) = data.split_at(RATE);
            self.absorb_block(block);
            data = rest;
        }

        if !data.is_empty() {
            self.buf[..data.len()].copy_from_slice(data);
            self.buf_len = data.len();
        }
    }

    fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), RATE);
        self.state.absorb_block(block);
        self.state.permute();
    }

    fn absorb_buffered_block(&mut self) {
        self.state.absorb_block(&self.buf[..RATE]);
        self.state.permute();
    }

    pub fn finalize(mut self, out: &mut [u8]) -> &[u8] {
        // Pad the remaining bytes in the buffer and absorb the final block.
        for b in &mut self.buf[self.buf_len..RATE] {
            *b = 0;
        }
        self.buf[self.buf_len] ^= SHA3_DOMAIN;
        self.buf[RATE - 1] ^= 0x80;
        self.absorb_buffered_block();

        self.state.squeeze_block(out);
        out
    }
}

impl<const RATE: usize, const OUTPUT_LEN: usize> Default
    for Sha3<RATE, OUTPUT_LEN>
{
    fn default() -> Self {
        Self::new()
    }
}

pub type Sha3_256 = Sha3<136, 32>;
pub type Sha3_512 = Sha3<72, 64>;

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    let mut out = [0u8; 32];
    h.update(data);
    h.finalize(&mut out);
    out
}

pub fn sha3_512(data: &[u8]) -> [u8; 64] {
    let mut h = Sha3_512::new();
    h.update(data);
    let mut out = [0u8; 64];
    h.finalize(&mut out);
    out
}
