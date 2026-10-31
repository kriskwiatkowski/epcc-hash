//! One `Board` per target, selected by a `board-*` Cargo feature, so the
//! `tests.rs`/`bench.rs` entry points in `runtime-common` can be written
//! once and shared across every runtime crate instead of duplicated per
//! board. Loosely modeled on ariel-os's per-board-file + compile-time
//! selection pattern (`ariel-os-boards`), without pulling in its `laze`
//! build orchestrator or `embassy`-based peripheral model, which would be
//! disproportionate for the handful of boards here.
#![no_std]

cfg_if::cfg_if! {
    if #[cfg(feature = "board-stm32h573i-dk")] {
        mod stm32h573i_dk;
        pub use stm32h573i_dk::Board;
    } else if #[cfg(feature = "board-qemu")] {
        mod qemu_cortex_m;
        pub use qemu_cortex_m::Board;
    } else {
        compile_error!("board-support: enable exactly one `board-*` feature");
    }
}
