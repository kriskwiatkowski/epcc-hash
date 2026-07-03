# EPCC Hash

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
cargo build -p demo-cortex-m3 --target thumbv7m-none-eabi
qemu-system-arm \
  -machine mps2-an385 \
  -cpu cortex-m3 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv7m-none-eabi/debug/cortex-m3-tests
```

**Cortex-M33**

```bash
cargo build -p demo-cortex-m33 --target thumbv8m.main-none-eabi
qemu-system-arm \
  -machine mps2-an521 \
  -cpu cortex-m33 \
  -nographic \
  -semihosting-config enable=on,target=native \
  -kernel target/thumbv8m.main-none-eabi/debug/cortex-m33-tests
```

Each test name is printed via semihosting and QEMU exits with success after
all tests pass.

## Running benchmarks on embedded (QEMU)

Replace the binary name with the `-bench` variant, e.g. for Cortex-M33:

```bash
cargo build -p demo-cortex-m33 --target thumbv8m.main-none-eabi
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
