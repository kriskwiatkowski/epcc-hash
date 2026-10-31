//! Host-side micro-benchmarks, useful during development when real
//! hardware isn't at hand. Runs the same `sha3_bench::BENCHMARKS` table
//! the embedded targets under `targets/` measure via the DWT cycle
//! counter, reporting cycles here too via `criterion-cycles-per-byte`
//! (the CPU's `RDTSC` timestamp counter on x86/x86_64) — but since
//! `bitinterleaving_armv8m`'s assembly only compiles under
//! `target_arch = "arm"`, this exercises the portable fallback path, not
//! the hand-written ARM assembly. `RDTSC` ticks at a fixed reference
//! frequency rather than the core's dynamic clock, so these numbers are
//! good for relative comparisons but aren't the retired-instruction
//! cycle counts DWT reports. For real on-device numbers, see
//! `runtime/targets/stm32h573i-dk`.
//!
//! Each `BENCHMARKS` entry takes an accumulator and returns one derived
//! from its result, so threading it through repeated calls
//! (`acc = f(acc)`) creates a real dependency from one call to the
//! next. Without that, an out-of-order CPU would overlap several
//! independent calls and this would report throughput rather than one
//! call's latency.

use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;
use sha3_bench::BENCHMARKS;
use std::hint::black_box;

fn bench_all(c: &mut Criterion<CyclesPerByte>) {
    for (name, f) in BENCHMARKS {
        c.bench_function(name, |b| {
            let mut acc = black_box(0x0123_4567u32);
            b.iter(|| {
                acc = f(black_box(acc));
                black_box(acc)
            });
        });
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench_all
}
criterion_main!(benches);
