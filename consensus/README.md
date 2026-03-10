# CreditChain Consensus

The `consensus/` workspace implements CreditChainBFT, the Jolteon-style
Byzantine fault tolerant protocol that orders transactions and finalizes blocks
for the network.

## Overview

Consensus gives the validator set the behavior of a single replicated database.
Validators receive transactions from mempool, execute proposals speculatively,
exchange votes, form quorum certificates, and commit a canonical chain of
blocks when the commit rule is satisfied.

CreditChainBFT is designed for:

- safety under Byzantine faults below one-third of voting power
- liveness under partial synchrony
- fast optimistic settlement with a three-chain commit path
- clean separation between liveness and safety rules

## Main Components

- `src/block_storage/`: in-memory block tree, execution results, and certificates
- `src/liveness/`: round progression, proposer behavior, and timeout handling
- `consensus-types/`: proposals, votes, quorum certificates, and related types
- `safety-rules/`: persistent voting rules that prevent equivocation

Important runtime roles include:

- `RoundManager`: processes proposals, votes, and local events
- `RoundState`: handles proposer selection, timeouts, and round changes
- `SafetyRules`: enforces safe voting and signing behavior across restarts

## Integration Points

Consensus sits in the middle of the node:

- pulls candidate transactions from [`../mempool/README.md`](../mempool/README.md)
- asks [`../execution/README.md`](../execution/README.md) to speculatively execute blocks
- commits finalized results into [`../storage/README.md`](../storage/README.md)
- exchanges messages over [`../network/README.md`](../network/README.md)

## Why It Matters Publicly

For website and docs purposes, this is the subsystem behind the CreditChain
claims around:

- Jolteon BFT consensus
- fast settlement and finality
- deterministic ordering for stablecoin and financial workflows

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../execution/README.md`](../execution/README.md)
- [`../mempool/README.md`](../mempool/README.md)
