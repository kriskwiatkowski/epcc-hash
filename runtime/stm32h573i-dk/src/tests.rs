#![no_std]
#![no_main]

mod logger;

use cortex_m_rt::entry;
use log::info;
#[allow(unused_imports)]
use stm32h5xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    logger::init();
    info!("STM32H573I-DK tests");

    info!("{:<40} {:>6}", "name", "result");
    info!("{:-<48}", "");
    for (name, f) in common::tests::TESTS {
        f();
        // Note it will panic if the test fails, so we only print "ok" if it passes.
        info!("{:<40} {:>6}", name, "ok");
    }
    info!("{:-<48}", "");
    info!("all tests passed");

    for _ in 0..200_000 {
        cortex_m::asm::nop();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
