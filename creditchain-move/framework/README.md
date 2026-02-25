---
id: CreditChain-framework
title: CreditChain Framework
custom_edit_url: https://github.com/libra2org/libra2-core/edit/main/CreditChain-move/CreditChain-framework/README.md
---

## The CreditChain Framework

The CreditChain Framework defines the standard actions that can be performed on-chain
both by the CreditChain VM---through the various prologue/epilogue functions---and by
users of the blockchain---through the allowed set of transactions. This
directory contains different directories that hold the source Move
modules and transaction scripts, along with a framework for generation of
documentation, ABIs, and error information from the Move source
files. See the [Layout](#layout) section for a more detailed overview of the structure.

## Documentation

Each of the main components of the CreditChain Framework and contributing guidelines are documented separately. See them by version below:

* *CreditChain tokens* - [main](https://github.com/libra2org/libra2-core/blob/main/creditchain-move/framework/creditchain-token/doc/overview.md), [testnet](https://github.com/libra2org/libra2-core/blob/testnet/creditchain-move/framework/creditchain-token/doc/overview.md), [devnet](https://github.com/libra2org/libra2-core/blob/devnet/creditchain-move/framework/creditchain-token/doc/overview.md)
* *CreditChain framework* - [main](https://github.com/libra2org/libra2-core/blob/main/creditchain-move/framework/creditchain-framework/doc/overview.md), [testnet](https://github.com/libra2org/libra2-core/blob/testnet/creditchain-move/framework/creditchain-framework/doc/overview.md), [devnet](https://github.com/libra2org/libra2-core/blob/devnet/creditchain-move/framework/creditchain-framework/doc/overview.md)
* *CreditChain stdlib* - [main](https://github.com/libra2org/libra2-core/blob/main/creditchain-move/framework/creditchain-stdlib/doc/overview.md), [testnet](https://github.com/libra2org/libra2-core/blob/testnet/creditchain-move/framework/creditchain-stdlib/doc/overview.md), [devnet](https://github.com/libra2org/libra2-core/blob/devnet/creditchain-move/framework/creditchain-stdlib/doc/overview.md)
* *Move stdlib* - [main](https://github.com/libra2org/libra2-core/blob/main/creditchain-move/framework/move-stdlib/doc/overview.md), [testnet](https://github.com/libra2org/libra2-core/blob/testnet/creditchain-move/framework/move-stdlib/doc/overview.md), [devnet](https://github.com/libra2org/libra2-core/blob/devnet/creditchain-move/framework/move-stdlib/doc/overview.md)

Follow our [contributing guidelines](CONTRIBUTING.md) and basic coding standards for the CreditChain Framework.

## Compilation and Generation

The documents above were created by the Move documentation generator for CreditChain. It is available as part of the CreditChain CLI. To see its options, run:
```shell
libra2 move document --help
```

The documentation process is also integrated into the framework building process and will be automatically triggered like other derived artifacts, via `cached-packages` or explicit release building.

## Running Move tests

To test our Move code while developing the CreditChain Framework, run `cargo test` inside this directory:

```
cargo test
```

(Alternatively, run `cargo test -p creditchain-framework` from anywhere.)

To skip the Move prover tests, run:

```
cargo test -- --skip prover
```

To filter and run **all** the tests in specific packages (e.g., `creditchain_stdlib`), run:

```
cargo test -- creditchain_stdlib --skip prover
```

(See tests in `tests/move_unit_test.rs` to determine which filter to use; e.g., to run the tests in `creditchain_framework` you must filter by `move_framework`.)

To **filter by test name or module name** in a specific package (e.g., run the `test_empty_range_proof` in `creditchain_stdlib::ristretto255_bulletproofs`), run:

```
TEST_FILTER="test_range_proof" cargo test -- creditchain_stdlib --skip prover
```

Or, e.g., run all the Bulletproof tests:
```
TEST_FILTER="bulletproofs" cargo test -- creditchain_stdlib --skip prover
```

To show the amount of time and gas used in every test, set env var `REPORT_STATS=1`.
E.g.,
```
REPORT_STATS=1 TEST_FILTER="bulletproofs" cargo test -- creditchain_stdlib --skip prover
```

Sometimes, Rust runs out of stack memory in dev build mode.  You can address this by either:
1. Adjusting the stack size

```
export RUST_MIN_STACK=4297152
```

2. Compiling in release mode

```
cargo test --release -- --skip prover
```

## Layout
The overall structure of the CreditChain Framework is as follows:

```
├── creditchain-framework                                 # Sources, testing and generated documentation for CreditChain framework component
├── creditchain-token                                 # Sources, testing and generated documentation for CreditChain token component
├── creditchain-stdlib                                 # Sources, testing and generated documentation for CreditChain stdlib component
├── move-stdlib                                 # Sources, testing and generated documentation for Move stdlib component
├── cached-packages                                 # Tooling to generate SDK from move sources.
├── src                                     # Compilation and generation of information from Move source files in the CreditChain Framework. Not designed to be used as a Rust library
├── releases                                    # Move release bundles
└── tests
```
