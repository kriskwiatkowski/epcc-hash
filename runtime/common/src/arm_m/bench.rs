#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    for (name, f) in common::bench::BENCHMARKS {
        hprintln!("running {}", name);
        // We don't know where generic build runs, so by default this doesn't measure execution
        f(0);
    }
    debug::exit(debug::EXIT_SUCCESS);
    loop {
        cortex_m::asm::wfi();
    }
}
