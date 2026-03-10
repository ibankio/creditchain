# CreditChain Configuration

The `config/` workspace defines how CreditChain validators, fullnodes, faucets,
and supporting services are configured. It is the source of truth for network
identity, storage paths, consensus behavior, networking, observability, and
secure backends.

## Scope

This layer governs:

- validator and fullnode roles
- peer topology and network addresses
- consensus, mempool, execution, and state sync parameters
- storage locations and pruning settings
- safety rules, logging, metrics, and secure key storage
- genesis and testnet generation workflows

## Directory Map

- `src/config/`: typed configuration structures and defaults
- `src/generator.rs`: helpers for building validator and fullnode config sets
- `src/keys.rs`: test-oriented key handling utilities
- `global-constants/`: shared operational constants

## Common Workflows

Configuration generation is centered around `config-builder`.

Typical commands from the repository root:

```bash
cargo run -p config-builder -- validator ...
cargo run -p config-builder -- full-node create ...
cargo run -p config-builder -- faucet ...
```

These flows are used to:

- bootstrap a validator set and genesis configuration
- add fullnodes to an existing deployment
- produce faucet or mint credentials for test environments

## Operational Guidance

- Keep production overrides small and intentional; defaults are tuned with
  releases and broad overrides can hide important upstream improvements.
- Treat validator and fullnode configs as deployment artifacts and manage them
  through secure storage and infrastructure automation.
- Align storage, network, and state-sync settings with the guidance in
  [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md).

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../storage/README.md`](../storage/README.md)
- [`../network/README.md`](../network/README.md)
