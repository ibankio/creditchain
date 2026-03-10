# CreditChain Execution

The `execution/` workspace turns ordered transactions into state transitions.
It executes Move code, computes transaction outputs, and produces the new
authenticated state that consensus can agree on and storage can persist.

## Responsibilities

- execute proposed blocks speculatively before they are committed
- commit finalized blocks into durable storage
- compute transaction outputs, events, write sets, and state roots
- parallelize compatible workloads through BlockSTM-style execution paths

## Directory Map

- `executor/`: core execution pipeline
- `executor-service/`: service interface used by other subsystems
- `executor-types/`: shared execution data types
- `block-partitioner/`: transaction grouping and scheduling helpers
- `executor-test-helpers/`: local and test-only execution utilities
- `executor-benchmark/`: performance measurement tools

## How It Fits Together

Consensus proposes a block, execution evaluates it against the parent state,
and the resulting outputs are attached to the proposal path. Once consensus
commits that path, execution and storage make the results durable and visible to
external readers.

This design matters for CreditChain because:

- stablecoin and settlement flows depend on deterministic execution
- parallel execution improves throughput without changing the programming model
- explorer and SDK clients rely on consistent post-commit views of state

## Integration Points

- [`../consensus/README.md`](../consensus/README.md) drives speculative execution
- [`../storage/README.md`](../storage/README.md) persists committed outputs
- [`../state-sync/README.md`](../state-sync/README.md) replays or synchronizes committed data
- [`../api/README.md`](../api/README.md) exposes committed results to clients

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/03_MOVE_MODULES_SPEC.md`](../docs/03_MOVE_MODULES_SPEC.md)
