#![no_std]
#![no_main]

use board_support::Board;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let mut board = Board::init();

    board.println(format_args!("{:<40} {:>6}", "name", "result"));
    board.println(format_args!("{:-<48}", ""));
    for (name, f) in common::tests::TESTS {
        f();
        // Note it will panic if the test fails, so we only print "ok" if it passes.
        board.println(format_args!("{:<40} {:>6}", name, "ok"));
    }
    board.println(format_args!("{:-<48}", ""));
    board.println(format_args!("all tests passed"));

    board.finish()
}
