#![no_std]
#![no_main]

mod logger;

use cortex_m_rt::entry;
#[allow(unused_imports)]
use stm32h5xx_hal::{pac, prelude::*};
use log::{info};

#[entry]
fn main() -> ! {
    logger::init();
    info!("Starting STM32H573I-DK demo...");
    //let summary = ;

    info!("Operations Run:");
    /*
    for operation in summary.operations.iter() {
        info!(" - {}", operation);
    }
    */

    // Give RTT chance to flush the logs before we go to sleep
    for _ in 0..200_000 { cortex_m::asm::nop(); }

    loop {
        // put core to sleep to save power, and to allow the ITM logs to be flushed
        cortex_m::asm::wfi();
    }
}
