# CreditChain Architecture Overview

> Document 01 | CreditChain Design Series | Version 1.0
> Companion to: OpenIBank Doc 24 (CreditChain Spec) & Doc 54 (CreditChain L1 Impl)

---

## 1. Executive Summary

CreditChain is the Layer-1 public blockchain powering the OpenIBank ecosystem — the world's
first AI-era crypto banking platform. Built as a strategic fork of CreditChain (itself descended
from Aptos → Diem → Libra), CreditChain inherits battle-tested BFT consensus, the Move
smart-contract language, and a high-performance parallel execution engine, then customizes
them for financial-grade settlement, stablecoin issuance, and cross-chain interoperability.

### Mission
Replace traditional banking settlement rails (SWIFT, ACH, Fedwire) with a deterministic,
sub-second, formally-verified public chain purpose-built for the AI financial era.

### Heritage Chain
```
Facebook Libra (2019) → Diem (2020) → Aptos (2022) → CreditChain (2024) → CreditChain (2025)
```

---

## 2. Strategic Position in OpenIBank Four Pillars

```
┌─────────────────────────────────────────────────────────────────┐
│                    OpenIBank Ecosystem                          │
├────────────────┬────────────────┬──────────────┬────────────────┤
│   Pillar 1     │   Pillar 2     │  Pillar 3    │  Pillar 4      │
│  Self-Banking  │   Exchange     │  Stablecoin  │  Public Chain  │
│  (iBank App)   │   (DAX)        │  (IUSD)      │ (CreditChain)  │
├────────────────┼────────────────┼──────────────┼────────────────┤
│ Wallets,Cards  │ BTC/ETH/CCC    │ Mint/Burn    │ Consensus      │
│ Payments,P2P   │ Spot+Margin    │ Reserves     │ Move VM        │
│ KYC/AML,Credit │ OrderBook      │ Peg Stability│ Settlement     │
│ AI Advisor     │ MDAX Engine    │ Audit Trail  │ Bridge Layer   │
└────────────────┴────────────────┴──────────────┴────────────────┘
                          │                │              │
                          └────────────────┴──────────────┘
                            All settle on CreditChain L1
```

CreditChain is the **settlement backbone** — every IUSD mint, every exchange trade
settlement, every self-banking transaction ultimately anchors to CreditChain.

---

## 3. CreditChain Foundation Analysis

### 3.1 Inherited Architecture

| Layer | Component | Technology | Status |
|-------|-----------|------------|--------|
| Consensus | Jolteon | HotStuff BFT + DAG ordering (Quorum Store) | Production-ready |
| Execution | Block Executor | Parallel execution with BlockSTM | Production-ready |
| Smart Contracts | Move VM | Linear type system, resource safety | Production-ready |
| Storage | RocksDB + JMT | Jellyfish Merkle Tree + Transaction Accumulator | Production-ready |
| Networking | libra2-network | Noise protocol, multiplexed RPC | Production-ready |
| API | REST + gRPC | OpenAPI spec, indexer gRPC streaming | Production-ready |
| State Sync | State Sync v2 | Snapshot + continuous sync | Production-ready |

### 3.2 Codebase Metrics

| Metric | Value |
|--------|-------|
| Total Rust crates | 200+ |
| Lines of Rust | ~500K+ |
| Lines of Move | ~150K+ |
| Workspace members | 200+ in Cargo.toml (990 lines) |
| Commit history | 23,059 commits (Aptos heritage) |
| Languages | Rust 63.6%, Move 32.4%, Python 2.4% |

### 3.3 Key Crate Families

```
crates/libra2-*         (66 crates)  → Core infrastructure
libra2-move/libra2-*    (24 crates)  → Move VM and tooling
libra2-move/framework/  (3 crates)   → Move standard library
consensus/              (3 crates)   → BFT consensus
execution/              (6 crates)   → Block execution
storage/                (8 crates)   → Persistent storage
network/                (6 crates)   → P2P networking
api/                    (4 crates)   → REST API layer
ecosystem/              (20 crates)  → Indexer, NFT, node-checker
```

---

## 4. CreditChain Customization Architecture

### 4.1 Fork Strategy: What Changes vs. What Stays

#### KEEP AS-IS (Core Infrastructure)
- Jolteon BFT consensus engine
- BlockSTM parallel execution
- Move VM core (bytecode verifier, type system, linear resources)
- RocksDB storage layer + Jellyfish Merkle Tree
- Noise protocol networking
- State synchronization
- Transaction accumulator / proof system

#### REBRAND (All 200+ crates)
- `libra2-*` → `creditchain-*` (crate names, binary names, config keys)
- Move framework addresses: `0x1` stays but module names update
- CLI tools: `libra2` CLI → `creditchain` CLI
- Docker images: `libra2-node` → `creditchain-node`
- Config files: `libra2-node.yaml` → `creditchain-node.yaml`

#### CUSTOMIZE (CreditChain-Specific)
- **Genesis**: New chain_id (0xCC01), CCC token, initial validator set
- **Token Economics**: CCC (CreditChain Coin) as native gas token
- **IUSD Module**: Move-native stablecoin with proof-carrying reserves
- **Settlement DvP**: Delivery-vs-Payment atomic settlement
- **Bridge**: Cross-chain bridge to Ethereum, BSC, Solana
- **WorldLine Anchoring**: Real-world event timestamping
- **AI Agent Framework**: On-chain AI agent execution proofs

#### ADD NEW (CreditChain Innovations)
- Credit scoring on-chain module
- Institutional settlement layer
- Regulatory compliance module (KYC attestations on-chain)
- Multi-asset clearing and netting
- Real-time reserve attestation for IUSD

### 4.2 Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                     CreditChain Node                             │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────────────┐  │
│  │  REST API    │  │  gRPC Index  │  │  WebSocket Events      │  │
│  │  /v1/*       │  │  Streaming   │  │  Real-time blocks      │  │
│  └──────┬───────┘  └──────┬───────┘  └───────────┬────────────┘  │
│         │                 │                      │               │
│  ┌──────┴─────────────────┴──────────────────────┴────────────┐  │
│  │                    API Gateway Layer                       │  │
│  └────────────────────────┬───────────────────────────────────┘  │
│                           │                                      │
│  ┌────────────────────────┴───────────────────────────────────┐  │
│  │              Mempool (Transaction Ordering)                │  │
│  └────────────────────────┬───────────────────────────────────┘  │
│                           │                                      │
│  ┌────────────────────────┴───────────────────────────────────┐  │
│  │         Jolteon Consensus (HotStuff BFT + DAG)             │  │
│  │  ┌──────────────┐  ┌────────────┐  ┌─────────────────────┐ │  │
│  │  │ Safety Rules │  │ Quorum Cert│  │  Epoch Management   │ │  │
│  │  └──────────────┘  └────────────┘  └─────────────────────┘ │  │
│  └────────────────────────┬───────────────────────────────────┘  │
│                           │                                      │
│  ┌────────────────────────┴───────────────────────────────────┐  │
│  │            Block Executor (BlockSTM Parallel)              │  │
│  │  ┌─────────────────────────────────────────────────────┐   │  │
│  │  │                    Move VM                          │   │  │
│  │  │  ┌───────────┐ ┌──────────┐ ┌─────────────────────┐ │   │  │
│  │  │  │creditchain│ │  IUSD    │ │  Settlement DvP     │ │   │  │
│  │  │  │-framework │ │  Module  │ │  Module             │ │   │  │
│  │  │  └───────────┘ └──────────┘ └─────────────────────┘ │   │  │
│  │  │  ┌──────────┐ ┌──────────┐ ┌──────────────────────┐ │   │  │
│  │  │  │  Bridge  │ │WorldLine │ │  Credit Score        │ │   │  │
│  │  │  │  Module  │ │  Anchor  │ │  Module              │ │   │  │
│  │  │  └──────────┘ └──────────┘ └──────────────────────┘ │   │  │
│  │  └─────────────────────────────────────────────────────┘   │  │
│  └────────────────────────┬───────────────────────────────────┘  │
│                           │                                      │
│  ┌────────────────────────┴───────────────────────────────────┐  │
│  │              Storage (RocksDB + Jellyfish Merkle)          │  │
│  │  ┌─────────────┐  ┌──────────────┐  ┌───────────────────┐  │  │
│  │  │ State Store │  │ Txn Accum.   │  │ Event Store       │  │  │
│  │  └─────────────┘  └──────────────┘  └───────────────────┘  │  │
│  └────────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐  │
│  │              P2P Network (Noise Protocol)                  │  │
│  │  Validator ←→ Validator mesh, Full-node sync               │  │
│  └────────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────────┘
```

---

## 5. CCC Token Economics

### 5.1 Token Overview

| Property | Value |
|----------|-------|
| Name | CreditChain Coin |
| Symbol | CCC |
| Total Supply | 1,000,000,000 (1 Billion) |
| Decimals | 8 (like Aptos APT) |
| Smallest Unit | 1 Octa = 0.00000001 CCC |
| Chain ID | 0xCC01 (52225) |

### 5.2 Token Allocation

| Category | Percentage | Amount | Vesting |
|----------|-----------|--------|---------|
| Ecosystem & Community | 30% | 300M | 4-year linear |
| Protocol Development | 25% | 250M | 4-year with 1-year cliff |
| Foundation Reserve | 20% | 200M | Governance-locked |
| Validators & Staking | 15% | 150M | Released per epoch |
| Exchange Liquidity | 5% | 50M | Immediate |
| Advisors & Partners | 5% | 50M | 3-year linear |

### 5.3 Gas Model

CreditChain inherits CreditChain/Aptos gas model:
- **Gas Unit Price**: Minimum 100 Octa per gas unit
- **Max Gas Per Transaction**: 2,000,000 units
- **Storage Fee**: Per-byte state storage deposit (refundable)
- **Execution Fee**: Per-instruction gas consumption
- **Fee burn**: 50% of gas fees burned, 50% to validators

---

## 6. Consensus: Jolteon BFT

### 6.1 Overview

CreditChain retains the Jolteon consensus protocol — a pipelined HotStuff BFT variant
with optimistic responsiveness and DAG-based transaction ordering via Quorum Store.

### 6.2 Key Parameters

| Parameter | CreditChain Value | Notes |
|-----------|-------------------|-------|
| Block time | 200ms target | Sub-second finality |
| Finality | 3 rounds (~600ms) | Optimistic path |
| Max validators | 100 (Phase 1) → 500 (Phase 3) | Permissioned → Open |
| Fault tolerance | f < n/3 | Byzantine fault tolerant |
| Epoch duration | 2 hours | Validator set rotation |
| Quorum Store batch | 500 txns | DAG ordering batches |

### 6.3 Validator Requirements (Phase 1)

| Resource | Minimum |
|----------|---------|
| CPU | 16 cores (AMD EPYC / Intel Xeon) |
| RAM | 64 GB |
| Storage | 2 TB NVMe SSD |
| Network | 1 Gbps dedicated |
| Stake | 1,000,000 CCC minimum |

---

## 7. Move Smart Contract Layer

### 7.1 CreditChain Framework Modules

The CreditChain Move framework extends the standard CreditChain framework with
financial-grade modules:

```move
// Core framework (inherited from libra2-framework)
0x1::creditchain_account      // Account management
0x1::creditchain_coin         // CCC native token
0x1::creditchain_governance   // On-chain governance
0x1::staking                  // Validator staking
0x1::block                    // Block metadata

// CreditChain Custom Modules (NEW)
0x1::iusd                     // IUSD stablecoin
0x1::settlement               // DvP atomic settlement
0x1::bridge                   // Cross-chain bridge
0x1::worldline                // Real-world event anchoring
0x1::credit_score             // On-chain credit scoring
0x1::compliance               // KYC/AML attestations
0x1::clearing                 // Multilateral netting
0x1::reserve_proof            // IUSD reserve attestation
```

### 7.2 IUSD Move Module (Simplified)

```move
module 0x1::iusd {
    use std::signer;
    use creditchain_framework::coin;

    /// IUSD token type
    struct IUSD has key, store, drop {}

    /// Reserve attestation — updated every 60 seconds
    struct ReserveAttestation has key {
        total_supply: u64,
        total_reserves_usd: u64,
        reserve_ratio_bps: u64,    // Basis points (10000 = 100%)
        last_attestation_epoch: u64,
        auditor_address: address,
    }

    /// Mint IUSD — only callable by authorized issuer
    public entry fun mint(
        issuer: &signer,
        to: address,
        amount: u64,
    ) acquires ReserveAttestation { ... }

    /// Burn IUSD — only callable by authorized issuer
    public entry fun burn(
        issuer: &signer,
        from: address,
        amount: u64,
    ) acquires ReserveAttestation { ... }

    /// Update reserve attestation — only callable by auditor
    public entry fun attest_reserves(
        auditor: &signer,
        total_reserves_usd: u64,
    ) acquires ReserveAttestation { ... }
}
```

### 7.3 Settlement DvP Module

```move
module 0x1::settlement {
    use std::signer;

    /// Atomic Delivery vs Payment
    struct DvPOrder has key, store {
        seller: address,
        buyer: address,
        asset_type: vector<u8>,
        asset_amount: u64,
        payment_amount: u64,    // in IUSD
        status: u8,             // 0=pending, 1=matched, 2=settled, 3=failed
        expiry_epoch: u64,
    }

    /// Execute atomic settlement
    public entry fun settle(
        settler: &signer,
        order_id: u64,
    ) acquires DvPOrder { ... }
}
```

---

## 8. Network Topology

### 8.1 Node Types

| Node Type | Role | Count (Phase 1) |
|-----------|------|-----------------|
| Validator Node (VN) | Consensus participation, block production | 7-21 |
| Validator Full Node (VFN) | State sync, transaction relay | 1 per validator |
| Public Full Node (PFN) | API serving, transaction submission | 50+ |
| Archive Node | Full history, indexer feeding | 3+ |
| Indexer Node | gRPC streaming, analytics | 5+ |

### 8.2 Network Phases

| Phase | Timeline | Validators | Governance |
|-------|----------|-----------|------------|
| Phase 0: Devnet | Month 1-3 | 4 (iBank-operated) | Centralized |
| Phase 1: Testnet | Month 4-6 | 7-21 (invited) | Permissioned |
| Phase 2: Mainnet Beta | Month 7-12 | 21-50 (staked) | Semi-decentralized |
| Phase 3: Mainnet | Month 13+ | 50-500 (open) | Fully decentralized |

---

## 9. Cross-Chain Bridge Architecture

### 9.1 Supported Chains (Priority Order)

| Chain | Bridge Type | Priority |
|-------|-------------|----------|
| Ethereum | Lock-and-Mint + Light Client | P0 |
| BSC | Lock-and-Mint | P1 |
| Solana | Message Passing | P1 |
| Bitcoin | Wrapped BTC (relay) | P2 |
| Polygon | Lock-and-Mint | P2 |
| Arbitrum/Optimism | Native bridge | P3 |

### 9.2 Bridge Flow

```
Source Chain                    CreditChain
┌───────────┐                  ┌──────────┐
│  Lock     │   Relay/Oracle   │  Mint    │
│  Asset    │ ───────────────→ │  Wrapped │
│  in Vault │                  │  Asset   │
└───────────┘                  └──────────┘

CreditChain                    Destination Chain
┌───────────┐                  ┌──────────┐
│  Burn     │   Relay/Oracle   │  Unlock  │
│  Wrapped  │ ───────────────→ │  Native  │
│  Asset    │                  │  Asset   │
└───────────┘                  └──────────┘
```

---

## 10. Performance Targets

| Metric | Target | Benchmark |
|--------|--------|-----------|
| TPS (simple transfer) | 10,000+ | Aptos: 160K theoretical |
| TPS (complex Move txn) | 2,000+ | Financial operations |
| Block time | 200ms | Sub-second UX |
| Finality | <1 second | 3-round BFT |
| State sync (full) | <4 hours | For new validators |
| API latency (p99) | <100ms | REST endpoint |
| Storage growth | ~50 GB/month | With pruning |

---

## 11. Security Model

### 11.1 Layers of Defense

1. **Move Type System**: Linear types prevent double-spend at the language level
2. **Bytecode Verifier**: All deployed modules verified before execution
3. **BFT Consensus**: Tolerates up to f < n/3 Byzantine validators
4. **Formal Verification**: Move Prover for critical framework modules
5. **Key Management**: Ed25519 + MultiEd25519 (multi-sig support)
6. **Account Model**: Sequence numbers prevent replay attacks
7. **Gas Metering**: Prevents resource exhaustion attacks

### 11.2 Cryptographic Primitives

| Primitive | Algorithm | Usage |
|-----------|-----------|-------|
| Signing | Ed25519 | Transaction signatures |
| Multi-sig | MultiEd25519 | Governance, bridge |
| Hashing | SHA-3-256 | State tree, txn hash |
| VRF | ECVRF-P256 | Leader election |
| BLS | BLS12-381 | Aggregate signatures (DKG) |
| Encryption | AES-256-GCM | Network layer (Noise) |

---

## 12. Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| CC-ARCH-1 | CreditChain MUST maintain BFT safety with f < n/3 Byzantine validators | Jolteon consensus |
| CC-ARCH-2 | All IUSD operations MUST be atomic (mint+reserve or fail) | Move VM |
| CC-ARCH-3 | Settlement DvP MUST be all-or-nothing | Move transaction |
| CC-ARCH-4 | Bridge operations MUST have timeout + refund path | Module logic |
| CC-ARCH-5 | CCC total supply MUST never exceed 1 billion | Genesis + module |
| CC-ARCH-6 | Gas fees MUST be denominated in CCC only | Framework config |
| CC-ARCH-7 | Validator stake MUST meet minimum before epoch inclusion | Staking module |
| CC-ARCH-8 | All state transitions MUST be deterministic and reproducible | Move VM + BlockSTM |

---

## 13. Document Cross-References

| Document | Relationship |
|----------|-------------|
| This Doc (01_ARCHITECTURE) | Architecture overview |
| 02_REBRANDING_GUIDE | CreditChain → CreditChain naming |
| 03_GENESIS_CONFIG | Chain genesis & token setup |
| 04_MOVE_MODULES | Custom Move module specs |
| 05_BRIDGE_SPEC | Cross-chain bridge design |
| 06_DEPLOYMENT_GUIDE | Node deployment & operations |
| OpenIBank Doc 24 | CreditChain high-level spec |
| OpenIBank Doc 54 | CreditChain L1 impl details |
| OpenIBank Doc 55 | Four Pillars strategy |

---

*CreditChain — The Settlement Backbone of AI-Era Finance*
