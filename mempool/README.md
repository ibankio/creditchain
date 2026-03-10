# CreditChain Mempool

The `mempool/` workspace manages transactions before they are ordered by
consensus. It is the intake, buffering, and distribution layer between API
clients and validators.

## Responsibilities

- accept signed transactions from node APIs and internal components
- perform lightweight admission checks before transactions are queued
- prioritize, store, and expire uncommitted transactions
- broadcast or share candidate transactions with peer nodes
- remove committed transactions after consensus finalizes them

## Why It Matters

Mempool is the first stop for most application traffic coming from wallets,
services, Creditscan deep links, and SDK-based integrations. Good mempool
behavior directly affects:

- transaction propagation latency
- validator fairness and throughput
- user-facing submission reliability

## Code Layout

- `src/`: admission, storage, networking hooks, and runtime logic

## Integration Points

- [`../api/README.md`](../api/README.md) submits transactions into mempool
- [`../consensus/README.md`](../consensus/README.md) pulls transactions for proposals
- [`../state-sync/README.md`](../state-sync/README.md) notifies mempool when committed transactions can be dropped

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
