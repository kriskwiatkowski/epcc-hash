//! Test runner for RV32 under `qemu-riscv32` (Linux user mode).
//!
//! Freestanding rather than newlib-based: the only environment this
//! needs is the `write` and `exit` syscalls, which keeps it independent
//! of a RISC-V libc being installed.

#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::{self, Write};

const SYS_WRITE: usize = 64;
const SYS_EXIT: usize = 93;

fn sys_exit(code: usize) -> ! {
    // SAFETY: the exit syscall does not return.
    unsafe {
        asm!("ecall", in("a7") SYS_EXIT, in("a0") code, options(noreturn))
    }
}

fn sys_write(buf: &[u8]) {
    // SAFETY: passing a valid pointer/length pair to write(2) on stdout.
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_WRITE,
            in("a0") 1usize,
            in("a1") buf.as_ptr(),
            in("a2") buf.len(),
            lateout("a0") _,
            options(nostack),
        );
    }
}

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(s.as_bytes());
        Ok(())
    }
}

macro_rules! println {
    ($($arg:tt)*) => {{
        let _ = writeln!(Stdout, $($arg)*);
    }};
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("FAILED: {}", info);
    sys_exit(1)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{:<40} {:>6}", "name", "result");
    println!("{:-<48}", "");
    for (name, f) in common::tests::TESTS {
        // A failing test panics, so reaching the print means it passed.
        f();
        println!("{:<40} {:>6}", name, "ok");
    }
    println!("{:-<48}", "");
    println!("all tests passed");
    sys_exit(0)
}
