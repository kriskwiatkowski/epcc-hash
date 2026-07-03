#![no_std]
#![no_main]

mod logger;

use cortex_m_rt::entry;
use log::info;
use stm32h5xx_hal::{pac, prelude::*};
const K_NUM_ITERS: u32 = 10;

#[entry]
fn main() -> ! {
    logger::init();
    info!("STM32H573I-DK benchmarks @ 32 MHz");

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.vos0().freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(32.MHz()).freeze(pwrcfg, &dp.SBS);
    let dwt = cp.DWT.constrain(cp.DCB, &ccdr.clocks);

    info!("{:<32} {:>12} {:>12}", "name", "cycles", "us");
    info!("{:-<58}", "");
    for (name, f) in common::bench::BENCHMARKS {
        f(); // warm-up
        let mut total_ticks = 0u64;
        let mut total_nanos = 0u64;
        for _ in 0..K_NUM_ITERS {
            let cd = dwt.measure(|| f());
            total_ticks += cd.as_ticks() as u64;
            total_nanos += cd.as_nanos() as u64;
        }
        info!(
            "{:<32} {:>12} {:>12}",
            name,
            total_ticks / K_NUM_ITERS as u64,
            total_nanos / K_NUM_ITERS as u64 / 1000
        );
    }
    info!("{:-<58}", "");

    for _ in 0..200_000 {
        cortex_m::asm::nop();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
