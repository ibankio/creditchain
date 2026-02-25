<a href="https://github.com/ibankio/creditchain">
  <img width="100%" src="./.assets/creditchain_github_banner.svg" alt="CreditChain — Institution-Grade Blockchain for AI-Era Finance" />
</a>

---

[![Status](https://img.shields.io/badge/status-proprietary-red.svg)](https://github.com/ibankio/creditchain)
[![Chain ID](https://img.shields.io/badge/chain_id-0xCC01-blue.svg)]()
[![Token](https://img.shields.io/badge/token-CCC-gold.svg)]()
[![Consensus](https://img.shields.io/badge/consensus-Jolteon_BFT-green.svg)]()
[![VM](https://img.shields.io/badge/smart_contracts-Move-purple.svg)]()

# CreditChain

**The institution-grade Layer 1 blockchain powering the AI-era financial infrastructure.**

CreditChain is the settlement backbone of the [OpenIBank](https://github.com/ibankio) ecosystem — purpose-built for stablecoins, digital asset settlement, cross-chain interoperability, and institutional-grade crypto banking. It delivers sub-second BFT finality, formal verification through the Move language, and enterprise deployment flexibility from public mainnet to private sovereign chains.

## The Four Pillars of AI-Era Crypto Banking

CreditChain serves as **Pillar 4** — the foundational settlement layer — in the OpenIBank ecosystem:

| Pillar | System | Role |
|--------|--------|------|
| 1. Self-Banking | iBank App | Wallets, payments, P2P, credit, AI advisor |
| 2. Exchange | DAX + MDAX | Bitcoin/crypto exchange with institutional matching engine |
| 3. Stablecoin | IUSD | Proof-carrying institutional stablecoin (100% reserve-backed) |
| **4. Public Chain** | **CreditChain** | **Settlement, consensus, Move VM, cross-chain bridge** |

Every IUSD mint, every exchange trade settlement, every self-banking transaction ultimately anchors to CreditChain.

## Core Capabilities

### Jolteon BFT Consensus
- Sub-second finality (~600ms, 3-round optimistic path)
- HotStuff BFT variant with DAG-based transaction ordering (Quorum Store)
- Byzantine fault tolerant: f < n/3
- 10,000+ TPS for simple transfers, 2,000+ for complex Move transactions

### Move Smart Contracts
- Linear type system prevents double-spend at the language level
- Bytecode verifier ensures safety before execution
- Formal verification via Move Prover for critical modules
- Resource-oriented programming for financial assets

### One-Click Stablecoin (Stablecoin-as-a-Service)

Any institution can create their own branded stablecoin in **one transaction**:

```
POST /v1/stablecoin/create
{ "name": "Acme Dollar", "symbol": "ACSD", "peg_currency": "USD" }
→ Fully operational stablecoin in < 2 seconds
```

Powered by Move's Fungible Asset V2 — no module deployment needed per coin.
**6 native stablecoins (the "Big 6") at genesis:** IUSD, IEUR, IJPY, IGBP, ICNY, ICAD
— covering ~88% of global forex volume with 15 cross-currency pairs (C(6,2)).
Unlimited additional stablecoins at runtime. Each gets: reserve attestation,
rate limiting, circuit breakers, compliance gating, cross-chain bridge, and
cross-stablecoin atomic swaps.

### CreditChain-Specific Modules

| Module | Purpose |
|--------|---------|
| `stablecoin_factory` | **One-Click Stablecoin creation** (Fungible Asset V2) |
| `stablecoin_registry` | Global stablecoin discovery & duplicate prevention |
| `stablecoin_swap` | Cross-stablecoin atomic swaps (same-peg 1:1 or oracle-priced) |
| `iusd_compat` | IUSD backward compatibility (wraps factory calls) |
| `ieur_compat` | IEUR backward compatibility |
| `ijpy_compat` | IJPY backward compatibility (0 decimals, JPY) |
| `igbp_compat` | IGBP backward compatibility |
| `icny_compat` | ICNY backward compatibility |
| `icad_compat` | ICAD backward compatibility |
| `settlement` | Atomic Delivery-vs-Payment (DvP) for financial settlement |
| `clearing` | Multilateral netting (reduces N obligations to ≤ N-1 transfers) |
| `bridge` | Cross-chain bridge (Ethereum, BSC, Solana, Bitcoin) — any stablecoin |
| `worldline` | Immutable real-world event anchoring for audit trails |
| `compliance` | On-chain KYC/AML attestation registry (zero PII on-chain) |
| `oracle` | Aggregated price feeds for multi-asset + forex valuation |
| `agent_registry` | AI agent registration, authorization, and execution proofs |
| `vault` | Multi-signature institutional custody |

### Enterprise Deployment Models

| Model | Validators | Use Case |
|-------|-----------|----------|
| Public Mainnet | 50-500 (open staking) | Global settlement |
| Consortium | 7-21 (invited institutions) | Inter-bank settlement |
| Private Enterprise | 4-7 (single organization) | Internal settlement |
| Sovereign | Government-operated | Central bank CBDC |
| Hybrid | Core + satellite nodes | Multi-national institutions |

## Token: CCC (CreditChain Coin)

| Property | Value |
|----------|-------|
| Symbol | CCC |
| Total Supply | 1,000,000,000 |
| Decimals | 8 |
| Chain ID (Mainnet) | 0xCC01 |
| Consensus | Jolteon BFT |
| Smart Contracts | Move |

## Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                     CreditChain Node                             │
├──────────────────────────────────────────────────────────────────┤
│  REST API / gRPC Indexer / WebSocket Events                      │
├──────────────────────────────────────────────────────────────────┤
│  Mempool → Jolteon Consensus (HotStuff BFT + DAG)                │
├──────────────────────────────────────────────────────────────────┤
│  Block Executor (BlockSTM Parallel Execution)                    │
│  ┌──────────────────────────────────────────────────────────┐    │
│  │  Move VM: Big 6 Stablecoins | Settlement | Bridge | ...  │    │
│  └──────────────────────────────────────────────────────────┘    │
├──────────────────────────────────────────────────────────────────┤
│  Storage: RocksDB + Jellyfish Merkle Tree + Transaction Accum.   │
├──────────────────────────────────────────────────────────────────┤
│  P2P Network: Noise Protocol (Validator Mesh + Full Node Sync)   │
└──────────────────────────────────────────────────────────────────┘
```

## Heritage

```
Facebook Libra (2019) → Diem (2020) → Aptos (2022) → Libra2 (2024) → CreditChain (2026)
```

CreditChain inherits battle-tested infrastructure from the Libra/Diem/Aptos lineage and extends it with institution-grade financial modules purpose-built for the AI financial era.

## Documentation

| Document | Description |
|----------|-------------|
| [01 Architecture](docs/01_CREDITCHAIN_ARCHITECTURE.md) | System architecture overview |
| [02 Token Economy Reference](docs/02_TOKEN_ECONOMY_REFERENCE.md) | CCC token economy reference example (non-binding, for user reference) |
| [03 Move Modules](docs/03_MOVE_MODULES_SPEC.md) | Custom Move module specifications (v2.0: StablecoinFactory) |
| [04 Bridge & Interop](docs/04_BRIDGE_AND_INTEROP_SPEC.md) | Cross-chain bridge design |
| [05 Deployment & Ops](docs/05_DEPLOYMENT_AND_OPERATIONS.md) | Node deployment & operations |
| [06 One-Click Stablecoin](docs/06_ONE_CLICK_STABLECOIN.md) | **Stablecoin-as-a-Service platform specification** |

Internal rebranding runbooks are maintained in `.prompt/` and intentionally excluded from repository publishing.

## Quick Start

```bash
# Build from source
cargo build --release -p creditchain-node

# Boot local 4-validator devnet
cargo run -p creditchain-localnet -- run \
    --num-validators 4 \
    --chain-id 52227 \
    --with-faucet

# Verify node health
curl http://localhost:8080/v1/-/healthy

# Get chain info
curl http://localhost:8080/v1/ | jq .
```

## Security

CreditChain employs defense-in-depth:

- **Move Type System**: Linear types eliminate double-spend at compile time
- **Bytecode Verifier**: All deployed code verified before execution
- **BFT Consensus**: Tolerates up to f < n/3 Byzantine validators
- **Formal Verification**: Move Prover for critical framework modules
- **Ed25519 + MultiEd25519**: Transaction signing with multi-sig support
- **Noise Protocol**: Encrypted validator-to-validator communication

Report security issues: [SECURITY.md](SECURITY.md)

## Implementation Roadmap

| Phase | Scope | Status |
|-------|-------|--------|
| P0 | Fork verification & build | Planned |
| P1 | Crate rebranding (200+ crates) | Planned |
| P2 | Genesis & CCC tokenomics | Planned |
| P3 | IUSD stablecoin module | Planned |
| P4 | Settlement DvP & clearing | Planned |
| P5 | Cross-chain bridge | Planned |
| P6 | Compliance, WorldLine, AI agents | Planned |
| P7 | Testnet launch & hardening | Planned |
| P8 | Enterprise deployment toolkit | Planned |
| P9 | **One-Click Stablecoin Factory** (FA V2) | Planned |

## Repository Structure

```
creditchain/
├── api/                    # REST API layer
├── consensus/              # Jolteon BFT consensus
├── crates/                 # Core infrastructure (66+ crates)
├── config/                 # Node configuration
├── docs/                   # Design documents
├── docker/                 # Docker deployment
├── execution/              # Block executor (BlockSTM)
├── creditchain-move/            # Move VM, framework, stdlib (pending rename)
├── creditchain-node/            # Node binary (pending rename)
├── mempool/                # Transaction mempool
├── network/                # P2P networking (Noise protocol)
├── storage/                # RocksDB + Jellyfish Merkle Tree
├── types/                  # Core type definitions
├── .prompt/                # Implementation system prompts (P0-P8)
└── Cargo.toml              # Workspace (200+ members)
```

## Contributing

CreditChain is proprietary enterprise infrastructure. Contributions are managed through the CreditChain Research Team.

- [Contributing Guide](CONTRIBUTING.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)
- [Rust Coding Style](RUST_CODING_STYLE.md)

## Proprietary Notice

CreditChain is proprietary enterprise banking and financial infrastructure developed and maintained by the CreditChain Research Team.

Copyright (c) CreditChain Research Team. All rights reserved.
