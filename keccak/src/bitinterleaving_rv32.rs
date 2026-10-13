//! Bit-interleaving conversions for RV32 with the Zbkb extension.
//!
//! Zbkb's `zip`/`unzip` are exactly the bit gather/scatter this needs,
//! and they exist only on RV32 -- which is precisely where interleaving
//! is used. `unzip` moves a word's even-indexed bits into the low half
//! and its odd-indexed bits into the high half; `zip` is the inverse.
//! With `pack` to glue two halves together, each conversion collapses
//! from the ~30-instruction butterfly the portable code needs to six
//! instructions.
//!
//! These are `asm!` rather than `global_asm!` + `extern`: at six
//! instructions a call would cost more in clobbered registers around it
//! than the conversion itself.

use core::arch::asm;

/// Split the 64-bit lane `(r1, r2)` into its even-bit and odd-bit words.
#[inline]
pub fn conv_into_bi(r1: u32, r2: u32) -> (u32, u32) {
    let even: u32;
    let odd: u32;
    // SAFETY: pure register-to-register arithmetic, no memory access.
    unsafe {
        asm!(
            ".option push",
            ".option arch, +zbkb",
            // lo/hi each become (odd half : even half) after unzip, so
            // the even word is the two low halves packed together and
            // the odd word the two high halves.
            "unzip {lo}, {lo}",
            "unzip {hi}, {hi}",
            "srli  {t0}, {lo}, 16",
            "srli  {t1}, {hi}, 16",
            "pack  {lo}, {lo}, {hi}",
            "pack  {hi}, {t0}, {t1}",
            ".option pop",
            lo = inout(reg) r1 => even,
            hi = inout(reg) r2 => odd,
            t0 = out(reg) _,
            t1 = out(reg) _,
            options(pure, nomem, nostack),
        );
    }
    (even, odd)
}

/// Recombine the even-bit and odd-bit words back into a 64-bit lane.
#[inline]
pub fn conv_from_bi(r1: u32, r2: u32) -> (u32, u32) {
    let lo: u32;
    let hi: u32;
    // SAFETY: pure register-to-register arithmetic, no memory access.
    unsafe {
        asm!(
            ".option push",
            ".option arch, +zbkb",
            // Regroup into per-word (odd half : even half) pairs, which
            // is the form `zip` expects.
            "srli {t0}, {even}, 16",
            "srli {t1}, {odd}, 16",
            "pack {even}, {even}, {odd}",
            "pack {odd}, {t0}, {t1}",
            "zip  {even}, {even}",
            "zip  {odd}, {odd}",
            ".option pop",
            even = inout(reg) r1 => lo,
            odd = inout(reg) r2 => hi,
            t0 = out(reg) _,
            t1 = out(reg) _,
            options(pure, nomem, nostack),
        );
    }
    (lo, hi)
}
