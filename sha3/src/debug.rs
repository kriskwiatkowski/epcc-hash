#[cfg(all(not(feature = "std"), feature = "semihosting-log"))]
pub fn log_debug(msg: &str) {
    let _ = cortex_m_semihosting::hprintln!("DBG: {}", msg);
}

#[cfg(feature = "std")]
pub fn log_debug(msg: &str) {
    println!("DBG: {}", msg);
}

#[cfg(not(any(feature = "std", feature = "semihosting-log")))]
pub fn log_debug(_msg: &str) {}
