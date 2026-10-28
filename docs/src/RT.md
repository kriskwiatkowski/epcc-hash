# Runtime Environments

The `runtime` directory contains configuration for embedded targets. Each runtime is a separate crate and `runtime/common` contains common code shared between them. The project is implemented so that the same tests can be run on the host via `cargo test` and on real/emulated hardware via the `runtime/*` binaries.


## Running tests on native

```bash
cargo test
```

The ARM assembly is replaced with an equivalent Rust implementation on
non-ARM targets, so the same tests run on any host machine.

## Running tests on embedded (QEMU)

Install the Rust targets:

```bash
rustup target add thumbv7m-none-eabi thumbv8m.main-none-eabi
```

Build and run for each supported core:

**Cortex-M3**

```bash
cargo build -p cortex-m3 --target thumbv7m-none-eabi
qemu-system-arm \
  -machine mps2-an385 \
  -cpu cortex-m3 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7m-none-eabi/debug/cortex-m3-tests
```

**Cortex-M33**

```bash
cargo build -p cortex-m33 --target thumbv8m.main-none-eabi
qemu-system-arm \
  -machine mps2-an521 \
  -cpu cortex-m33 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv8m.main-none-eabi/debug/cortex-m33-tests
```

Each test name is printed via semihosting and QEMU exits with success after
all tests pass.

## Running benchmarks on the host

```bash
cargo bench -p runtime-host
```

Useful during development when real hardware isn't at hand: runs the same
`sha3_bench::BENCHMARKS` table as the embedded runtimes, via
[criterion](https://github.com/bheisler/criterion.rs). On x86/x86_64,
[criterion-cycles-per-byte](https://github.com/wainwrightmark/criterion-cycles-per-byte)
is used to report cycles (read via the `RDTSC` timestamp counter) rather
than wall-clock time, so results are comparable in kind to the DWT cycle
counts from real hardware — though `RDTSC` ticks at a fixed reference
frequency rather than the core's dynamic clock, so treat these as relative
numbers, not exact retired-instruction cycle counts. Since
`bitinterleaving_armv8m`'s assembly only compiles under
`target_arch = "arm"`, on a non-ARM host this measures the portable
fallback path on both sides of each comparison, not the hand-written ARM
assembly — for real on-device numbers, see
[Running on STM32H573I-DK](#running-on-stm32h573i-dk) below. An HTML report
is written to `target/criterion/report/index.html`.

### Getting accurate cycle counts on X86

Under CPU frequency boost, `RDTSC` ticks fall behind the core's actual
clock - it increments at a fixed reference rate (usually the CPU's base/
"nominal" frequency), so when the core briefly runs faster, it retires
more real work per tick than at that reference rate. Reported "cycles"
on a boosting machine therefore undercount the true per-call cost.

To get numbers that line up with real core cycles, pin the CPU to its
nominal (non-boost) frequency before running the benchmark. Find that
frequency first:

```bash
cpupower frequency-info
```

Look for the `Nominal Frequency` line under "amd-pstate limits" (AMD) or
the equivalent base-clock figure for your CPU, then pin to it:

```bash
# Disable turbo/boost globally
sudo sh -c 'echo 0 > /sys/devices/system/cpu/cpufreq/boost'

# Use the performance governor (no powersave throttling)
sudo cpupower frequency-set -g performance

# Pin min = max to the nominal frequency, e.g. 2.00 GHz
sudo cpupower frequency-set -d 2000000 -u 2000000
```

This is a system-wide change — it affects every process until reverted,
and will reduce peak performance while active. Revert with:

```bash
sudo cpupower frequency-set -d <cpuinfo_min_freq> -u <cpuinfo_max_freq>
sudo cpupower frequency-set -g powersave
sudo sh -c 'echo 1 > /sys/devices/system/cpu/cpufreq/boost'
```

## Running benchmarks on embedded (QEMU)

Replace the binary name with the `-bench` variant, e.g. for Cortex-M33:

```bash
cargo build -p cortex-m33 --target thumbv8m.main-none-eabi
qemu-system-arm \
  -machine mps2-an521 \
  -cpu cortex-m33 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv8m.main-none-eabi/debug/cortex-m33-bench
```

## Running on STM32H573I-DK

Requires [probe-rs](https://probe.rs/). Build and flash from the `runtime/stm32h573i-dk` directory:

```bash
cd runtime/stm32h573i-dk
```

**Tests**

```bash
cargo build --bin stm32h573i-dk-tests
probe-rs run --chip STM32H573IIKx ../../target/thumbv8m.main-none-eabi/debug/stm32h573i-dk-tests
```

**Benchmarks**

```bash
cargo build --bin stm32h573i-dk-bench
probe-rs run --chip STM32H573IIKx ../../target/thumbv8m.main-none-eabi/debug/stm32h573i-dk-bench
```

Output is printed over RTT. `probe-rs run` flashes the binary and attaches to RTT automatically.
