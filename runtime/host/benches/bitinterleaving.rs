//! Host-side micro-benchmarks, useful during development when real
//! hardware isn't at hand. Runs the same `sha3_bench::BENCHMARKS` table
//! the embedded runtimes under `runtime/` measure via the DWT cycle
//! counter — but since `bitinterleaving_armv8m`'s assembly only compiles
//! under `target_arch = "arm"`, this exercises the portable fallback
//! path, not the hand-written ARM assembly. For real on-device numbers,
//! see `runtime/stm32h573i-dk`.

use criterion::{criterion_group, criterion_main, Criterion};
use sha3_bench::BENCHMARKS;

fn bench_all(c: &mut Criterion) {
    for (name, f) in BENCHMARKS {
        c.bench_function(name, |b| b.iter(|| f()));
    }
}

criterion_group!(benches, bench_all);
criterion_main!(benches);
