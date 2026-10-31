#![no_std]
#![no_main]

use board_support::Board;
use cortex_m_rt::entry;

const K_NUM_ITERS: u32 = 10;

#[entry]
fn main() -> ! {
    let mut board = Board::init();

    board.println(format_args!("{:<32} {:>12} {:>12}", "name", "cycles", "us"));
    board.println(format_args!("{:-<58}", ""));
    for (name, f) in common::bench::BENCHMARKS {
        let mut acc = f(0); // warm-up
        match board.measure_repeated(*f, &mut acc, K_NUM_ITERS) {
            Some((cycles, nanos)) => {
                board.println(format_args!(
                    "{:<32} {:>12} {:>12}",
                    name,
                    cycles,
                    nanos / 1000
                ));
            }
            // Board doesn't measure cycles (e.g. QEMU); it already ran `f` once above.
            None => board.println(format_args!("{:<32} {:>12} {:>12}", name, "-", "-")),
        }
    }
    board.println(format_args!("{:-<58}", ""));

    board.finish()
}
