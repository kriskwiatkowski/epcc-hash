//! Board support for QEMU's `mps2-an385` (Cortex-M3) and `mps2-an521`
//! (Cortex-M33) machines: output over semihosting, and exit the emulator
//! when done rather than idling forever.

use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

pub struct Board;

impl Board {
    pub fn init() -> Self {
        Board
    }

    pub fn println(&mut self, args: core::fmt::Arguments) {
        hprintln!("{}", args);
    }

    /// QEMU's instruction timing isn't representative of real hardware, so
    /// this runs `f` once and reports no cycle count, same as the former
    /// per-board `bench.rs` did.
    pub fn measure_repeated(
        &mut self,
        f: fn(u32) -> u32,
        acc: &mut u32,
        _iters: u32,
    ) -> Option<(u64, u64)> {
        *acc = f(*acc);
        None
    }

    pub fn finish(&mut self) -> ! {
        debug::exit(debug::EXIT_SUCCESS);
        loop {
            cortex_m::asm::wfi();
        }
    }
}
