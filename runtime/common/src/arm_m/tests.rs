#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("{:<40} {:>6}", "name", "result");
    hprintln!("{:-<48}", "");
    for (name, f) in common::tests::TESTS {
        f();
        // Note it will panic if the test fails, so we only print "ok" if it passes.
        hprintln!("{:<40} {:>6}", name, "ok");
    }
    hprintln!("{:-<48}", "");
    hprintln!("all tests passed");

    debug::exit(debug::EXIT_SUCCESS);
    loop {
        cortex_m::asm::wfi();
    }
}
