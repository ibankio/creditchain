# CreditChain Networking

The `network/` workspace provides the peer-to-peer transport layer for
CreditChain nodes. It handles how validators and fullnodes discover each other,
authenticate peers, exchange protocol messages, and maintain connectivity under
load.

## Directory Map

- `builder/`: helpers for assembling network stacks in node services
- `discovery/`: peer discovery and topology maintenance
- `framework/`: shared abstractions for protocol handlers and message routing
- `netcore/`: low-level transport primitives
- `memsocket/`: in-memory transport utilities used in tests and local workflows
- `benchmark/`: networking benchmarks and measurements

## Responsibilities

- establish authenticated, encrypted peer channels
- manage seed peers, upstream peers, and validator/fullnode topology
- register protocol handlers for consensus, mempool, and state sync traffic
- provide backpressure-aware messaging and connection management

## Production Notes

- Validator networking should stay tightly controlled and authenticated.
- Public traffic should normally terminate at fullnodes and API layers, not at
  the validator mesh.
- Network tuning should be coordinated with consensus, mempool, and state sync
  expectations during testnet and mainnet rollout.

## Integration Points

- [`../consensus/README.md`](../consensus/README.md)
- [`../mempool/README.md`](../mempool/README.md)
- [`../state-sync/README.md`](../state-sync/README.md)
- [`../config/README.md`](../config/README.md)

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
