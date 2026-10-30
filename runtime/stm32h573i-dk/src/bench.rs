#![no_std]
#![no_main]

mod logger;

use cortex_m_rt::entry;
use embedded_io::Write as _;
use log::info;
use stm32h5xx_hal::{pac, prelude::*};
const K_NUM_ITERS: u32 = 10;

// Second, independent output path alongside RTT: this board's ST-LINK VCP is
// USART1 on PA9 (TX) / PA10 (RX) at 115200 8N1. RTT needs the debugger to
// actively poll over SWD and has proven unreliable (lost RTT reattachment
// mid-session); UART just streams bytes over USB-serial regardless of
// debugger state, so keep both.
macro_rules! log_both {
    ($uart:expr, $($arg:tt)*) => {{
        info!($($arg)*);
        let _ = writeln!($uart, $($arg)*);
    }};
}

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

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let tx = gpioa.pa9.into_alternate();
    let rx = gpioa.pa10.into_alternate();
    let mut uart = dp
        .USART1
        .serial((tx, rx), 115200.Hz(), ccdr.peripheral.USART1, &ccdr.clocks)
        .unwrap();
    let _ = writeln!(uart, "UART up (USART1, PA9/PA10, 115200 8N1)");

    log_both!(uart, "{:<32} {:>12} {:>12}", "name", "cycles", "us");
    log_both!(uart, "{:-<58}", "");
    for (name, f) in common::bench::BENCHMARKS {
        let mut acc = f(0); // warm-up
        let mut total_ticks = 0u64;
        let mut total_nanos = 0u64;
        for _ in 0..K_NUM_ITERS {
            let cd = dwt.measure(|| acc = f(acc));
            total_ticks += cd.as_ticks() as u64;
            total_nanos += cd.as_nanos() as u64;
        }
        log_both!(
            uart,
            "{:<32} {:>12} {:>12}",
            name,
            total_ticks / K_NUM_ITERS as u64,
            total_nanos / K_NUM_ITERS as u64 / 1000
        );
    }
    log_both!(uart, "{:-<58}", "");

    for _ in 0..200_000 {
        cortex_m::asm::nop();
    }

    loop {
        cortex_m::asm::wfi();
    }
}
