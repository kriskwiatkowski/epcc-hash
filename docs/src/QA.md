# Quality Assurance
Here we discuss testing strategies for the project.

## Testing strategy
The crate is implemented in a way, so that it can be tested on embedded targets. The test driver is a seprate create which imports the `epcc-hash` library, links with one of the runtime environments from `runtimes/` directory and runs the tests. For that to work, some of the internal modules must re-exported. The re-exporting is done via feature flag called `testing`.

### KAT testing
The library uses [katwalk](https://github.com/kriskwiatkowski/katwalk) crate to
run the Known Answer Tests (KATs) from the [test vectors](https://github.com/kriskwiatkowski/https://github.com/kriskwiatkowski/crypto-test-vectors).

To download KAT tests and run the KATs, use the following command:

```bash
cargo xtask kat
```

### Testing crate layout
* `keccak-test` ([`sha3/test/ut`](../../sha3/test/ut)) is a separate crate so the same `#[test]` checks can run two ways:
    - on the host via `cargo test` (plain `#[test]` fns), and
    - on real/emulated hardware via the `runtime/*` binaries, which call the same checks through the `TESTS`/`BENCHMARKS` plain-fn tables instead of the `#[test]` harness (no-std targets can't run `cargo test` directly).

* [`sha3/test/kat`](../../sha3/test/kat) covers known-answer tests against the FIPS-202 vectors, driven by `cargo xtask kat` (clones the vector repo, builds a `katwalk`-compatible wrapper binary, runs the comparison).

## Validation of correctness
The correcness of implementation is a first and most basic check that project performs.

