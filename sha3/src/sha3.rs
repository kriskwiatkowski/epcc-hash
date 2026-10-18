use crate::keccak_32::*;
use crate::Permute;
use crate::{conv_from_bi, conv_into_bi};

const SHA3_DOMAIN: u8 = 0x06;

// Largest rate among SHA3-224/256/384/512, used to size the internal
// block buffer.
const MAX_RATE: usize = 144;

/// Streaming SHA3 hasher. `RATE` is the sponge rate in bytes and
/// `OUTPUT_LEN` the digest length in bytes; use the [`Sha3_224`],
/// [`Sha3_256`], [`Sha3_384`], [`Sha3_512`] aliases rather than naming
/// this type directly.
pub struct Sha3<const RATE: usize, const OUTPUT_LEN: usize> {
    // Bit-interleaved state: lane `i`'s even/odd halves live at
    // `state[2 * i]`/`state[2 * i + 1]` (see `permutation_32`).
    state: [u32; 50],
    buf: [u8; MAX_RATE],
    buf_len: usize,
}

impl<const RATE: usize, const OUTPUT_LEN: usize> Sha3<RATE, OUTPUT_LEN> {
    pub fn new() -> Self {
        const {
            assert!(RATE > 0 && RATE <= MAX_RATE && RATE % 8 == 0);
            assert!(OUTPUT_LEN > 0 && OUTPUT_LEN <= RATE);
        }
        Self { state: [0u32; 50], buf: [0u8; MAX_RATE], buf_len: 0 }
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

    pub fn finalize(mut self) -> [u8; OUTPUT_LEN] {
        for b in &mut self.buf[self.buf_len..RATE] {
            *b = 0;
        }
        self.buf[self.buf_len] ^= SHA3_DOMAIN;
        self.buf[RATE - 1] ^= 0x80;
        self.absorb_buffered_block();

        let mut out = [0u8; OUTPUT_LEN];
        for (i, chunk) in out.chunks_mut(8).enumerate() {
            let (lo, hi) =
                conv_from_bi(self.state[2 * i], self.state[2 * i + 1]);
            let mut lane = [0u8; 8];
            lane[..4].copy_from_slice(&lo.to_le_bytes());
            lane[4..].copy_from_slice(&hi.to_le_bytes());
            chunk.copy_from_slice(&lane[..chunk.len()]);
        }
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

pub type Sha3_224 = Sha3<144, 28>;
pub type Sha3_256 = Sha3<136, 32>;
pub type Sha3_384 = Sha3<104, 48>;
pub type Sha3_512 = Sha3<72, 64>;

pub fn sha3_224(data: &[u8]) -> [u8; 28] {
    let mut h = Sha3_224::new();
    h.update(data);
    h.finalize()
}

pub fn sha3_256(data: &[u8]) -> [u8; 32] {
    let mut h = Sha3_256::new();
    h.update(data);
    h.finalize()
}

pub fn sha3_384(data: &[u8]) -> [u8; 48] {
    let mut h = Sha3_384::new();
    h.update(data);
    h.finalize()
}

pub fn sha3_512(data: &[u8]) -> [u8; 64] {
    let mut h = Sha3_512::new();
    h.update(data);
    h.finalize()
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::path::Path;

    #[test]
    fn fips202_vectors_validate_hashml_dsa_sha3_and_shake_prehashes() {
        // SHA3-256 tcId=221 and SHA3-512 tcId=384 from the local FIPS-202
        // corpus. SHAKE-128 tcId=230 provides the 256-bit HashML-DSA
        // prehash, while SHAKE-256 tcId=35 provides at least its required
        // 512-bit prehash output.
        let cases = [
            (
                "SHA3-256",
                "",
                "A7FFC6F8BF1ED76651C14756A061D662F580FF4DE43B49FA82D80A4B80F8434A",
            ),
            (
                "SHA3-512",
                "",
                "A69F73CCA23A9AC5C8B567DC185A756E97C982164FE25859E0D1DCC1475C80A615B2123AF1F5F94C11E3E9402C3AC558F500199D95B6D3E301758586281DCD26",
            ),
            (
                "SHAKE-128",
                "B3E0ABDFE024B4D76D24F0",
                "6065987CB4865BAA5374CBE2803DC16228EE0FFDE023EA26F0D8CFC632DDD423",
            ),
            (
                "SHAKE-256",
                "A21198252B3928D4273BDC3BE3F3E026BAB4F52008491A1814EBBE9C39FBEA87118CD8993EA307582AA8C2ADBFB9336F29BF7A",
                "78C4EC2BA13DAF27FA4E67B78BC70CFF3FBA1B09157C5639BD2FF751BCCE48CD3200BE88554A988CF7A11767A9155DEA750CB909B11B9A1DFB9F49B875C8EFE3",
            ),
        ];

        for (hash_alg, message, expected_digest) in cases {
            let message = hex::decode(message).unwrap();
            let encoded =
                prehashed_message(&message, &[], hash_alg.as_bytes()).unwrap();
            // All FIPS 204 HashML-DSA OIDs used here are 11-byte DER values.
            assert_eq!(
                hex::encode(&encoded[13..]),
                expected_digest.to_ascii_lowercase(),
                "{hash_alg} pre-hash did not match its FIPS-202 KAT"
            );
        }
    }

    fn fips202_digest(
        algorithm: &str,
        message: &[u8],
        output_len: usize,
    ) -> Vec<u8> {
        match algorithm {
            "SHA3-256" => Sha3_256::digest(message).to_vec(),
            "SHA3-512" => Sha3_512::digest(message).to_vec(),
            "SHAKE-128" => {
                let mut output = vec![0; output_len];
                Shake128::default()
                    .chain(message)
                    .finalize_xof()
                    .read(&mut output);
                output
            }
            "SHAKE-256" => {
                let mut output = vec![0; output_len];
                Shake256::default()
                    .chain(message)
                    .finalize_xof()
                    .read(&mut output);
                output
            }
            _ => unreachable!("test only calls registered FIPS-202 algorithms"),
        }
    }

    #[test]
    fn fips202_byte_aligned_aft_vectors_when_configured() {
        let Ok(root) = std::env::var("ACVP_FIPS202_DIR") else {
            return;
        };
        let root = Path::new(&root);
        let algorithms = ["SHA3-256", "SHA3-512", "SHAKE-128", "SHAKE-256"];
        let mut tested = 0;

        for algorithm in algorithms {
            let prompt: Value = serde_json::from_slice(
                &std::fs::read(root.join(algorithm).join("prompt.json"))
                    .unwrap_or_else(|e| {
                        panic!(
                            "failed to read {algorithm} FIPS-202 prompt: {e}"
                        )
                    }),
            )
            .unwrap_or_else(|e| {
                panic!("failed to parse {algorithm} FIPS-202 prompt: {e}")
            });
            let expected: Value = serde_json::from_slice(
                &std::fs::read(root.join(algorithm).join("expectedResults.json")).unwrap_or_else(
                    |e| panic!("failed to read {algorithm} FIPS-202 expected results: {e}"),
                ),
            )
            .unwrap_or_else(|e| {
                panic!("failed to parse {algorithm} FIPS-202 expected results: {e}")
            });
            let expected_groups = expected["testGroups"]
                .as_array()
                .expect("FIPS-202 expected results must contain testGroups");

            for group in prompt["testGroups"]
                .as_array()
                .expect("FIPS-202 prompt must contain testGroups")
            {
                if group["testType"].as_str() != Some("AFT") {
                    continue;
                }
                let group_id = group["tgId"]
                    .as_u64()
                    .expect("FIPS-202 AFT group must contain tgId");
                let expected_group = expected_groups
                    .iter()
                    .find(|candidate| {
                        candidate["tgId"].as_u64() == Some(group_id)
                    })
                    .unwrap_or_else(|| {
                        panic!(
                            "missing expected group {group_id} for {algorithm}"
                        )
                    });

                for test in group["tests"]
                    .as_array()
                    .expect("FIPS-202 AFT group must contain tests")
                {
                    let message_len = test["len"]
                        .as_u64()
                        .expect("FIPS-202 AFT test must contain len");
                    let output_len = test["outLen"].as_u64().unwrap_or(0);
                    if message_len % 8 != 0
                        || (output_len != 0 && output_len % 8 != 0)
                    {
                        continue;
                    }
                    let test_id = test["tcId"]
                        .as_u64()
                        .expect("FIPS-202 AFT test must contain tcId");
                    let expected_test = expected_group["tests"]
                        .as_array()
                        .expect("FIPS-202 expected group must contain tests")
                        .iter()
                        .find(|candidate| candidate["tcId"].as_u64() == Some(test_id))
                        .unwrap_or_else(|| {
                            panic!("missing expected test {group_id}/{test_id} for {algorithm}")
                        });
                    let message = hex::decode(
                        test["msg"]
                            .as_str()
                            .expect("FIPS-202 AFT test must contain msg"),
                    )
                    .expect("FIPS-202 AFT message must be hexadecimal");
                    let expected_output = hex::decode(
                        expected_test["md"]
                            .as_str()
                            .expect("FIPS-202 expected test must contain md"),
                    )
                    .expect("FIPS-202 expected digest must be hexadecimal");
                    assert_eq!(
                        fips202_digest(algorithm, &message, expected_output.len()),
                        expected_output,
                        "{algorithm} FIPS-202 AFT mismatch at {group_id}/{test_id}"
                    );
                    tested += 1;
                }
            }
        }
        assert!(tested > 0, "no byte-aligned FIPS-202 AFT cases were found");
    }
}
*/
