# CreditChain Storage

The `storage/` workspace implements CreditChain's durable data layer. It owns
the ledger database, authenticated state structures, backup tooling, and
storage-facing interfaces used by the rest of the node.

## Directory Map

- `creditchaindb/`: primary ledger and state persistence
- `jellyfish-merkle/`: authenticated state tree implementation
- `accumulator/`: transaction accumulator data structures
- `schemadb/`: schema and storage abstractions
- `storage-interface/`: APIs used by execution and state sync
- `backup/`: backup coordinator, backup service, and restore tooling
- `db-tool/`: local database inspection and maintenance utilities
- `indexer/` and `indexer_schemas/`: internal indexing support
- `scratchpad/`: in-memory state views used during execution

## Responsibilities

- persist committed transactions, outputs, events, and proofs
- maintain authenticated state and historical ledger access
- serve data to APIs, state sync, and internal consumers
- manage backup and restore workflows
- apply pruning policies to keep nodes healthy over time

## Operational Notes

- Storage configuration lives under `config/src/config/storage_config.rs`.
- Backups are for durability and disaster recovery; they are not the preferred
  mechanism for bootstrapping ordinary new nodes.
- State sync is generally the right path for bringing up fresh fullnodes.
- Creditscan production deployments should rely on the dedicated indexer stack;
  the internal indexer is not a substitute for the full explorer topology.

## Integration Points

- [`../execution/README.md`](../execution/README.md) writes committed outputs
- [`../state-sync/README.md`](../state-sync/README.md) reads and persists synchronized data
- [`../api/README.md`](../api/README.md) serves committed ledger views to clients

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../docs/07_CREDITSCAN_BROWSER_GUIDE.md`](../docs/07_CREDITSCAN_BROWSER_GUIDE.md)
