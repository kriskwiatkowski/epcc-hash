//! One `Board` per target, selected by a `board-*` Cargo feature, so a
//! project's `tests.rs`/`bench.rs` entry points can be written once against
//! `Board::{init, println, measure_repeated, finish}` and shared across
//! every board/emulator instead of duplicated per target. Loosely modeled
//! on ariel-os's per-board-file + compile-time selection pattern
//! (`ariel-os-boards`), without pulling in its `laze` build orchestrator or
//! `embassy`-based peripheral model, which would be disproportionate here.
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "board-stm32h573i-dk")] {
        mod stm32h573i_dk;
        pub use stm32h573i_dk::Board;
    } else if #[cfg(feature = "board-semihosting")] {
        mod semihosting;
        pub use semihosting::Board;
    } else {
        compile_error!("runtime: enable exactly one `board-*` feature");
    }
}
