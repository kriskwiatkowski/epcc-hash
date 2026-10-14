// ACVP binary-protocol wrapper around the RustCrypto ml-dsa crate.
// See katwalk::modulewrapper for the shared stdin/stdout framing.

use katwalk::modulewrapper;
use serde::Serialize;
use sha3::Sha3;

use env_logger::{Builder, Target};
use log::LevelFilter;
use log::{error, info};
use std::fs::OpenOptions;

const UNSUPPORTED: &[u8] = b"unsupported";
const PARAMETER_SETS: [&str; 3] = ["SHA3-256"];

#[derive(Serialize)]
struct Registration {
    #[serde(rename = "algorithm")]
    algorithm: &'static [&'static str],
    #[serde(rename = "messageLength")]
    message_length: [LengthDomain; 1],
    #[serde(rename = "performLargeDataTest")]
    performLargeDataTest: bool,
}

#[derive(Serialize)]
struct LengthDomain {
    #[serde(rename = "min")]
    min: u32,
    #[serde(rename = "max")]
    max: u32,
    #[serde(rename = "increment")]
    increment: u32,
}

fn config() -> Result<Vec<u8>, String> {
    serde_json::to_vec(&vec![Registration {
        algorithm: &PARAMETER_SETS,
        message_length: [LengthDomain { min: 8, max: 65_536, increment: 8 }],
        performLargeDataTest: false,
    }])
    .map_err(|e| format!("failed to serialize ML-DSA configuration: {e}"))
}

fn expect_args<'a>(
    args: &'a [Vec<u8>],
    expected: usize,
    command: &str,
) -> Result<&'a [Vec<u8>], String> {
    if args.len() != expected {
        return Err(format!(
            "{command} expects {} arguments, got {}",
            expected - 1,
            args.len().saturating_sub(1)
        ));
    }
    Ok(args)
}

fn hash_sum(
    param_set: &str,
    message: &[u8],
    digest: &mut [u8],
) -> Result<Vec<u8>, String> {
    let c = Sha3_256::new();
    c.update(message);
    c.finalize(digest)
        .map_err(|e| format!("HASH sum computation failed: {e}"))?;
    Ok(digest.to_vec())
}

/*
fn sign_external(
    param_set: &str,
    sk: &[u8],
    message: &[u8],
    context: &[u8],
    pre_hash: &[u8],
    hash_alg: &[u8],
    _rnd: &[u8],
) -> Result<Vec<Vec<u8>>, String> {
    let p = MLDSAParameters::new(param_set)
        .map_err(|e| format!("invalid parameter set {param_set}: {e}"))?;
    let mut signature = vec![0; p.signature_length];
    let signature_len = match pre_hash {
        b"pure" => {
            if !hash_alg.is_empty() {
                return Err(
                    "hashAlg must be absent for pure ML-DSA".to_string()
                );
            }
            if context.len() > u8::MAX as usize {
                return Err(format!(
                    "context must be at most 255 bytes, got {}",
                    context.len()
                ));
            }
            // FIPS 204 Algorithm 2 (ML-DSA.Sign): M' = 0x00 || len(ctx) || ctx || M.
            // `sign` is the internal operation and expects this framing already
            // applied, so it must be built here rather than passing `message` raw.
            let mut m_prime = vec![0u8, context.len() as u8];
            m_prime.extend_from_slice(context);
            m_prime.extend_from_slice(message);
            // Only deterministic case supported for pure ML-DSA
            sign(&p, sk, &m_prime, true, &mut signature)
        }
        b"preHash" => {
            let hash_alg = std::str::from_utf8(hash_alg)
                .map_err(|e| format!("invalid hash algorithm: {e}"))?;
            hash_ml_dsa_sign(
                &p,
                sk,
                message,
                context,
                true,
                hash_alg,
                &mut signature,
            )
            .map_err(|e| format!("ML-DSA pre-hash signing failed: {e}"))?
        }
        _ => {
            return Err(format!(
                "unsupported ML-DSA preHash mode: {}",
                String::from_utf8_lossy(pre_hash)
            ));
        }
    };
    if signature_len != p.signature_length {
        return Err(format!(
            "signature length mismatch: got {}, expected {}",
            signature_len, p.signature_length
        ));
    }
    Ok(vec![signature])
}
*/

fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("fips202-wrapper.log")
        .expect("failed to open log file");

    Builder::new()
        .filter_level(LevelFilter::Debug)
        .target(Target::Pipe(Box::new(log_file)))
        .init();

    modulewrapper::run("fips202", |args| {
        let result = dispatch(args);
        if let Err(error) = &result {
            error!("Module request failed: {error}");
        }
        result
    });
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
