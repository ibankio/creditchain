# CreditChain State Sync

The `state-sync/` workspace keeps validators and fullnodes aligned with the
latest committed CreditChain state. It is responsible for catching lagging
nodes up to the network without requiring manual replay of the entire chain.

## Core Subsystems

- `state-sync-driver/`: drives sync progress and verifies incoming data
- `data-streaming-service/`: creates streaming views over transactions and state data
- `creditchain-data-client/`: fetches data from peers on behalf of the driver
- `storage-service/`: serves data to peers from local storage
- `inter-component/`: notifications between state sync, mempool, and other services

## Responsibilities

- detect when a node is behind
- fetch committed transactions or state snapshots from peers
- verify data before it is persisted locally
- notify other subsystems when new committed data becomes available

## Why It Matters

State sync is critical for:

- bootstrapping new fullnodes
- recovering from node downtime
- validator catch-up after maintenance or network issues
- supporting healthy data availability across the network

It complements, but does not replace, long-term backup and restore workflows in
[`../storage/README.md`](../storage/README.md).

## Integration Points

- [`../storage/README.md`](../storage/README.md) provides the local persistence layer
- [`../mempool/README.md`](../mempool/README.md) receives commit notifications
- [`../network/README.md`](../network/README.md) transports sync requests and responses

## Related Docs

- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../storage/README.md`](../storage/README.md)
