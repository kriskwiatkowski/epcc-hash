// ACVP binary-protocol wrapper around the RustCrypto ml-dsa crate.
// See katwalk::modulewrapper for the shared stdin/stdout framing.

use katwalk::modulewrapper;
use serde::Serialize;

use env_logger::{Builder, Target};
use epcc_keccak::sha3::Sha3_256;
use log::LevelFilter;
use log::{error, info};
use std::fs::OpenOptions;

// TODO: Move to katwalk
const UNSUPPORTED: &[u8] = b"unsupported";
const PARAMETER_SETS: [&str; 1] = ["SHA3-256"];

#[derive(Serialize)]
struct Registration {
    #[serde(rename = "algorithm")]
    algorithm: &'static str,
    #[serde(rename = "messageLength")]
    message_length: [LengthDomain; 1],
    #[serde(rename = "performLargeDataTest")]
    perform_large_data_test: [u32; 2],
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

macro_rules! REGISTER {
    ($algorithm:expr) => {
        Registration {
            algorithm: $algorithm,
            message_length: [LengthDomain {
                min: 0,
                max: 65_536,
                increment: 8,
            }],
        }
    };
}

fn config() -> Result<Vec<u8>, String> {
    serde_json::to_vec(&vec![REGISTER!("SHA3-256")])
        .map_err(|e| format!("failed to serialize ML-DSA configuration: {e}"))
}

// TODO: Move to katwalk
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

fn hash_sum(message: &[u8]) -> Result<Vec<u8>, String> {
    let mut c = Sha3_256::new();
    let digest: [u8; 32];
    c.update(message);
    digest = c.finalize();
    Ok(digest.to_vec())
}

fn hash_sum_mct(message: &[u8]) -> Result<Vec<u8>, String> {
    let mut m = message.to_vec();
    for _ in 0..1000u32 {
        let digest = hash_sum(&m)?;
        m = digest.clone();
    }
    Ok(m)
}

fn hash_sum_large_data_set(
    message: &[u8],
    in_len: u64,
) -> Result<Vec<u8>, String> {
    let n_blocks = in_len / message.len() as u64;
    let n_rem = in_len - n_blocks * message.len() as u64;

    let mut c = Sha3_256::new();
    for _ in 0..n_blocks {
        c.update(message);
    }
    if n_rem > 0 {
        c.update(&message[0..n_rem as usize]);
    }
    Ok(c.finalize().to_vec())
}

fn dispatch(args: &[Vec<u8>]) -> Result<Vec<u8>, String> {
    // TODO: Move to katwalk
    let command = std::str::from_utf8(args.first().ok_or("empty frame")?)
        .map_err(|e| format!("invalid command: {e}"))?;

    match command {
        "getConfig" => {
            expect_args(args, 1, command)?;
            Ok(config()?)
        }
        // TODO: is in param set
        "SHA3-256" => {
            let digest = hash_sum(&args[1])?;
            Ok(digest)
        }
        "SHA3-256/MCT" => {
            let digest = hash_sum_mct(&args[1])?;
            Ok(digest)
        }
        "SHA3-256/LDT" => {
            let in_len = u64::from_le_bytes(
                args[2]
                    .as_slice()
                    .try_into()
                    .map_err(|_| "invalid input length")?,
            );
            let digest = hash_sum_large_data_set(&args[1], in_len)?;
            Ok(digest)
        }
        _ => Err(format!("unknown command: {command}")),
    }
}

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

    modulewrapper::run("fips202_wrapper", |args| {
        let result = dispatch(args);
        if let Err(error) = &result {
            error!("Module request failed: {error}");
        }
        Ok(vec![result.unwrap_or_else(|_| UNSUPPORTED.to_vec())])
    });
}
