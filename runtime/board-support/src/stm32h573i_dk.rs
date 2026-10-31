//! Board support for the STM32H573I-DK: RTT and UART (USART1, PA9/PA10,
//! 115200 8N1) output side by side -- RTT needs the debugger to actively
//! poll over SWD and has proven unreliable mid-session (lost
//! reattachment), UART keeps streaming regardless of debugger state -- and
//! real DWT cycle counting at a 32 MHz sys clock.

mod logger;

use embedded_io::Write as _;
use log::info;
use stm32h5xx_hal::dwt::Dwt;
use stm32h5xx_hal::serial::Serial;
use stm32h5xx_hal::{pac, prelude::*};

pub struct Board {
    dwt: Dwt,
    uart: Serial<pac::USART1>,
}

impl Board {
    pub fn init() -> Self {
        logger::init();

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

        Board { dwt, uart }
    }

    pub fn println(&mut self, args: core::fmt::Arguments) {
        info!("{}", args);
        let _ = writeln!(self.uart, "{}", args);
    }

    pub fn measure_repeated(
        &mut self,
        f: fn(u32) -> u32,
        acc: &mut u32,
        iters: u32,
    ) -> Option<(u64, u64)> {
        let mut total_ticks = 0u64;
        let mut total_nanos = 0u64;
        for _ in 0..iters {
            let cd = self.dwt.measure(|| *acc = f(*acc));
            total_ticks += cd.as_ticks() as u64;
            total_nanos += cd.as_nanos() as u64;
        }
        Some((total_ticks / iters as u64, total_nanos / iters as u64))
    }

    pub fn finish(&mut self) -> ! {
        for _ in 0..200_000 {
            cortex_m::asm::nop();
        }
        loop {
            cortex_m::asm::wfi();
        }
    }
}
