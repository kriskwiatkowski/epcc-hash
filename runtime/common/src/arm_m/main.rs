#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use panic_semihosting as _;

//use demo_keygen::{
//};

#[entry]
fn main() -> ! {
    /*
    let summary = run_demo();

    hprintln!("Operations Run:");
    for operation in summary.operations.iter() {
        hprintln!(" - {}", operation);
    }
    */
    debug::exit(debug::EXIT_SUCCESS);
    loop {
        cortex_m::asm::wfi();
    }
}
