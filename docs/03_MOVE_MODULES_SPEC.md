# CreditChain Custom Move Modules Specification

> Document 03 | CreditChain Design Series | Version 2.0
> Scope: All CreditChain-specific Move modules deployed on the L1
> v2.0 Change: IUSD replaced by StablecoinFactory (Fungible Asset V2). See Doc 06.

---

## 1. Module Registry

CreditChain extends the inherited framework with institution-grade financial modules.
All custom modules deploy at address `0x1` as part of the CreditChain Framework.

| Module | Address | Purpose | Priority |
|--------|---------|---------|----------|
| `stablecoin_factory` | 0x1 | **One-Click Stablecoin creation** (Fungible Asset V2) | P0 |
| `stablecoin_registry` | 0x1 | **Global stablecoin discovery & duplicate prevention** | P0 |
| `stablecoin_swap` | 0x1 | **Cross-stablecoin atomic swaps** | P1 |
| `iusd_compat` | 0x1 | IUSD backward-compatibility wrapper (routes to factory) | P0 |
| `ieur_compat` | 0x1 | IEUR backward-compatibility wrapper | P0 |
| `ijpy_compat` | 0x1 | IJPY backward-compatibility wrapper | P0 |
| `igbp_compat` | 0x1 | IGBP backward-compatibility wrapper | P0 |
| `icny_compat` | 0x1 | ICNY backward-compatibility wrapper | P0 |
| `icad_compat` | 0x1 | ICAD backward-compatibility wrapper | P0 |
| `settlement` | 0x1 | Delivery-vs-Payment atomic settlement | P0 |
| `clearing` | 0x1 | Multilateral netting engine | P1 |
| `bridge` | 0x1 | Cross-chain asset bridge (any factory stablecoin) | P1 |
| `worldline` | 0x1 | Real-world event anchoring & timestamping | P1 |
| `credit_score` | 0x1 | On-chain institutional credit scoring | P2 |
| `compliance` | 0x1 | KYC/AML attestation registry | P2 |
| `agent_registry` | 0x1 | AI agent execution proof & registry | P3 |
| `oracle` | 0x1 | Price oracle aggregation (incl. forex for cross-stablecoin) | P1 |
| `vault` | 0x1 | Institutional custody vault | P2 |
| `governance_ext` | 0x1 | Extended governance (institutional voting) | P2 |

> **Architecture Change (v2.0):** The original `iusd` and `reserve_proof` modules are
> replaced by `stablecoin_factory` + `stablecoin_registry`. IUSD becomes the first
> instance created by the factory at genesis. Any institution can create additional
> branded stablecoins via `stablecoin_factory::create_stablecoin()` in a single
> transaction. See `docs/06_ONE_CLICK_STABLECOIN.md` for full specification.

---

## 2. Stablecoin Factory & Registry (One-Click Stablecoin)

> **v2.0 Architectural Upgrade**: Replaced hardcoded `iusd` module with
> `stablecoin_factory` using Fungible Asset V2. Any institution can create
> branded stablecoins at runtime — no module deployment needed.
> See `docs/06_ONE_CLICK_STABLECOIN.md` for the full 800-line specification.

### 2.1 Design Principles

- **One-Click Creation**: Single `create_stablecoin()` call → fully operational coin
- **Fungible Asset V2**: Object-based dynamic asset creation (NOT old Coin&lt;T&gt;)
- **Proof-Carrying Money**: Every stablecoin unit traces to a verified reserve attestation
- **Institutional-Grade**: KYC_INSTITUTIONAL required to create; authorized issuers to mint
- **60-Second Reconciliation**: On-chain supply matches off-chain reserves within 60s
- **Compliance-First**: Every operation recorded with purpose, counterparty, jurisdiction
- **IUSD as First Instance**: IUSD created at genesis via factory — not a special case

### 2.2 Key Modules

#### StablecoinFactory (`0x1::stablecoin_factory`)

The factory module creates and manages ALL stablecoins on CreditChain.
Each stablecoin is a Fungible Asset V2 Object with deterministic address.

**Entry Functions:**

| Function | Purpose | Access |
|----------|---------|--------|
| `create_stablecoin(creator, name, symbol, decimals, peg, ...)` | One-click creation | KYC_INSTITUTIONAL |
| `mint(issuer, metadata_addr, recipient, amount, purpose)` | Mint tokens | Authorized issuer |
| `burn(issuer, metadata_addr, from, amount, purpose)` | Burn tokens | Authorized issuer |
| `attest_reserves(auditor, metadata_addr, reserves, proof_hash)` | Reserve attestation | Authorized auditor |
| `set_paused(admin, metadata_addr, paused)` | Circuit breaker | Admin |
| `add_issuer(admin, metadata_addr, new_issuer)` | Issuer management | Admin |
| `remove_issuer(admin, metadata_addr, issuer)` | Issuer management | Admin |
| `add_auditor(admin, metadata_addr, new_auditor)` | Auditor management | Admin |
| `freeze_account(admin, metadata_addr, account)` | Compliance freeze | Admin |
| `update_rate_limits(admin, metadata_addr, ...)` | Rate limit config | Admin |
| `transfer_admin(admin, metadata_addr, new_admin)` | Admin transfer | Admin |

**Per-Stablecoin Object Resources:**

```
Object @ 0xSTABLECOIN_ADDR
├── Metadata (FA V2: name, symbol, decimals, supply)
├── ManagedRefs (MintRef, BurnRef, TransferRef)
├── StablecoinConfig (peg_currency, reserve params, fees, creator)
├── StablecoinGovernance (admin, issuers, auditors, paused, frozen_accounts)
├── ReserveAttestation (reserves, ratio, proof_hash, auditor, timestamp)
└── RateTracker (epoch volumes, limits, single-tx limits)
```

#### StablecoinRegistry (`0x1::stablecoin_registry`)

Global registry tracking all stablecoins. Prevents duplicate symbols.

| Function | Purpose | Access |
|----------|---------|--------|
| `register(...)` | Register new stablecoin | Factory only (friend) |
| `symbol_exists(symbol)` | Check duplicate | View (public) |
| `get_by_symbol(symbol)` | Lookup metadata addr | View (public) |
| `total_count()` | Count all stablecoins | View (public) |
| `list(offset, limit)` | Paginated listing | View (public) |
| `list_by_peg(peg_currency)` | Filter by peg | View (public) |

#### StablecoinSwap (`0x1::stablecoin_swap`)

Cross-stablecoin atomic swaps.

| Function | Purpose |
|----------|---------|
| `swap_same_peg(sender, from, to, amount)` | 1:1 swap (same peg currency) |
| `swap(sender, from, to, amount, min_out)` | Oracle-priced swap (different pegs) |

#### Native Currency Compatibility Modules

Each of the Big 6 native stablecoins gets a direct-access wrapper:

| Module | Symbol | Wraps factory calls with hardcoded metadata address |
|--------|--------|-----------------------------------------------------|
| `0x1::iusd_compat` | IUSD | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |
| `0x1::ieur_compat` | IEUR | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |
| `0x1::ijpy_compat` | IJPY | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |
| `0x1::igbp_compat` | IGBP | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |
| `0x1::icny_compat` | ICNY | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |
| `0x1::icad_compat` | ICAD | `mint`, `burn`, `total_supply`, `reserve_ratio_bps` |

### 2.3 Native Big 6 Genesis Stablecoins

CreditChain launches with 6 native stablecoins covering ~88% of global forex volume:

| # | Symbol | Name | Peg | ISO 4217 | Decimals |
|---|--------|------|-----|----------|----------|
| 1 | **IUSD** | iBank US Dollar | USD | 840 | 6 |
| 2 | **IEUR** | iBank Euro | EUR | 978 | 6 |
| 3 | **IJPY** | iBank Japanese Yen | JPY | 392 | 0 |
| 4 | **IGBP** | iBank British Pound | GBP | 826 | 6 |
| 5 | **ICNY** | iBank Chinese Yuan | CNY | 156 | 6 |
| 6 | **ICAD** | iBank Canadian Dollar | CAD | 124 | 6 |

All created at genesis via factory, all 100% reserve-backed (IJPY uses 0 decimals per ISO 4217):

```move
// Genesis initialization — Big 6 native stablecoins
stablecoin_registry::initialize(framework);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank US Dollar"),       utf8(b"IUSD"), 6, utf8(b"USD"), ...);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank Euro"),            utf8(b"IEUR"), 6, utf8(b"EUR"), ...);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank Japanese Yen"),    utf8(b"IJPY"), 0, utf8(b"JPY"), ...);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank British Pound"),   utf8(b"IGBP"), 6, utf8(b"GBP"), ...);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank Chinese Yuan"),    utf8(b"ICNY"), 6, utf8(b"CNY"), ...);
stablecoin_factory::create_stablecoin(framework, utf8(b"iBank Canadian Dollar"), utf8(b"ICAD"), 6, utf8(b"CAD"), ...);
// Registry: #1 IUSD, #2 IEUR, #3 IJPY, #4 IGBP, #5 ICNY, #6 ICAD
// 15 cross-pairs (C(6,2)) available for atomic forex swaps via oracle
```

Each native currency also gets a compat module (`iusd_compat`, `ieur_compat`,
`ijpy_compat`, `igbp_compat`, `icny_compat`, `icad_compat`) for direct symbol-specific access.
See `docs/06_ONE_CLICK_STABLECOIN.md` Section 5 for full details.

    /// ──────────────────────────────────────────
    /// Auditor operations
    /// ──────────────────────────────────────────

    /// Submit reserve attestation with off-chain proof hash.
    /// Called every 60 seconds by authorized auditor.
    public entry fun attest_reserves(
        auditor: &signer,
        total_reserves_usd: u64,
        proof_hash: vector<u8>,
    ) acquires IUSDConfig, ReserveAttestation { }

    /// ──────────────────────────────────────────
    /// Admin operations
    /// ──────────────────────────────────────────

    /// Emergency pause — halts all mint/burn
    public entry fun set_paused(
        authority: &signer,
        paused: bool,
    ) acquires IUSDConfig { }

    /// Add authorized issuer
    public entry fun add_issuer(
        authority: &signer,
        new_issuer: address,
    ) acquires IUSDConfig { }

    /// Remove authorized issuer
    public entry fun remove_issuer(
        authority: &signer,
        issuer_to_remove: address,
    ) acquires IUSDConfig { }

    /// ──────────────────────────────────────────
    /// View functions
    /// ──────────────────────────────────────────

    #[view]
    public fun total_supply(): u64 acquires IUSDConfig { }

    #[view]
    public fun reserve_ratio_bps(): u64 acquires ReserveAttestation { }

    #[view]
    public fun is_paused(): bool acquires IUSDConfig { }

    #[view]
    public fun last_attestation_age_secs(): u64 acquires ReserveAttestation { }
}
```

### 2.4 Stablecoin Factory Invariants

| ID | Invariant |
|----|-----------|
| OCS-1 | Symbol MUST be unique across ALL stablecoins (registry enforced) |
| OCS-2 | Fresh reserve attestation (< max_age) REQUIRED for minting |
| OCS-3 | Reserve ratio MUST remain ≥ min_ratio after every mint |
| OCS-4 | `total_supply` of each stablecoin MUST equal sum of all holder balances (FA V2 guarantees) |
| OCS-5 | Only authorized issuers MAY call mint/burn for each stablecoin |
| OCS-6 | Only authorized auditors MAY submit attestations for each stablecoin |
| OCS-7 | Pause MUST block ALL operations except unpause |
| OCS-8 | Every operation MUST emit event + WorldLine anchor |
| OCS-9 | Frozen accounts MUST NOT receive or send tokens |
| OCS-10 | Cross-stablecoin swaps MUST be atomic (single Move transaction) |
| OCS-11 | Platform fees MUST be collected before token delivery |
| OCS-12 | IUSD MUST be first entry in StablecoinRegistry (genesis guarantee) |
| OCS-13 | Rate limits MUST reset at epoch boundary |
| OCS-14 | KYC_INSTITUTIONAL (level 4) REQUIRED to create new stablecoin |
| OCS-15 | Creation fee (CCC) MUST be paid to platform treasury |

---

## 3. Settlement DvP Module

### 3.1 Design Principles

- **Atomic**: Delivery and payment happen in the same transaction or not at all
- **Multi-Asset**: Settle CCC, IUSD, or any registered coin type
- **Institutional**: Supports bilateral and multilateral settlement
- **Netting**: Integrates with clearing module for net settlement

### 3.2 Module Definition

```move
module 0x1::settlement {
    use std::signer;
    use std::string::String;
    use creditchain_framework::timestamp;

    /// Settlement status codes
    const STATUS_PENDING: u8 = 0;
    const STATUS_MATCHED: u8 = 1;
    const STATUS_SETTLED: u8 = 2;
    const STATUS_FAILED: u8 = 3;
    const STATUS_EXPIRED: u8 = 4;
    const STATUS_CANCELLED: u8 = 5;

    /// Error codes
    const E_NOT_AUTHORIZED: u64 = 100;
    const E_ORDER_NOT_FOUND: u64 = 101;
    const E_ORDER_EXPIRED: u64 = 102;
    const E_INSUFFICIENT_FUNDS: u64 = 103;
    const E_INVALID_STATUS: u64 = 104;
    const E_COUNTERPARTY_MISMATCH: u64 = 105;

    /// ──────────────────────────────────────────
    /// Core types
    /// ──────────────────────────────────────────

    /// A Delivery-vs-Payment settlement order
    struct DvPOrder has key, store, copy, drop {
        order_id: u64,
        seller: address,
        buyer: address,
        /// Type identifier for asset being delivered
        asset_type: String,
        asset_amount: u64,
        /// Payment in IUSD (micro-units)
        payment_amount: u64,
        status: u8,
        created_at: u64,
        expiry_epoch: u64,
        /// Settlement reference for audit trail
        settlement_ref: String,
    }

    /// Batch settlement for multilateral netting
    struct BatchSettlement has key, store {
        batch_id: u64,
        orders: vector<u64>,
        net_amounts: vector<NetPosition>,
        status: u8,
        settled_at: u64,
    }

    /// Net position after multilateral netting
    struct NetPosition has store, copy, drop {
        participant: address,
        net_payable: u64,      // IUSD owed
        net_receivable: u64,   // IUSD due
        net_direction: bool,   // true = net payer, false = net receiver
    }

    /// ──────────────────────────────────────────
    /// Settlement operations
    /// ──────────────────────────────────────────

    /// Create a new DvP order
    public entry fun create_order(
        seller: &signer,
        buyer: address,
        asset_type: String,
        asset_amount: u64,
        payment_amount: u64,
        expiry_epochs: u64,
        settlement_ref: String,
    ) { }

    /// Match an order — buyer confirms willingness
    public entry fun match_order(
        buyer: &signer,
        order_id: u64,
    ) acquires DvPOrder { }

    /// Execute atomic settlement — both legs or neither
    public entry fun settle(
        settler: &signer,
        order_id: u64,
    ) acquires DvPOrder { }

    /// Cancel a pending/matched order
    public entry fun cancel_order(
        canceller: &signer,
        order_id: u64,
    ) acquires DvPOrder { }

    /// Execute batch settlement (multilateral netting result)
    public entry fun settle_batch(
        authority: &signer,
        batch_id: u64,
    ) acquires BatchSettlement { }

    /// ──────────────────────────────────────────
    /// View functions
    /// ──────────────────────────────────────────

    #[view]
    public fun get_order(order_id: u64): DvPOrder acquires DvPOrder { }

    #[view]
    public fun get_batch(batch_id: u64): BatchSettlement acquires BatchSettlement { }

    #[view]
    public fun total_settled_volume(): u64 { }
}
```

### 3.3 Settlement Invariants

| ID | Invariant |
|----|-----------|
| SET-1 | DvP settlement MUST be atomic — both legs execute or neither |
| SET-2 | Expired orders MUST NOT be settleable |
| SET-3 | Only seller can create, only buyer can match |
| SET-4 | Batch settlement net positions MUST sum to zero |
| SET-5 | Every settlement MUST emit event with full audit trail |

---

## 4. Clearing & Netting Module

### 4.1 Design

Implements multilateral netting to reduce gross settlement obligations.
Based on OpenIBank MDAX netting algorithms (Doc 10 / Doc 53).

```move
module 0x1::clearing {
    use std::string::String;

    /// A clearing cycle processes all pending obligations
    struct ClearingCycle has key, store {
        cycle_id: u64,
        participants: vector<address>,
        gross_obligations: vector<Obligation>,
        net_positions: vector<NetPosition>,
        netting_efficiency_bps: u64,  // basis points saved
        status: u8,
        created_at: u64,
        settled_at: u64,
    }

    struct Obligation has store, copy, drop {
        from: address,
        to: address,
        amount: u64,
        currency: String,
        reference: String,
    }

    struct NetPosition has store, copy, drop {
        participant: address,
        net_amount: u64,
        direction: bool, // true = pay, false = receive
    }

    /// Submit obligation to clearing house
    public entry fun submit_obligation(
        submitter: &signer,
        counterparty: address,
        amount: u64,
        currency: String,
        reference: String,
    ) { }

    /// Execute netting calculation for current cycle
    public entry fun execute_netting(
        authority: &signer,
        cycle_id: u64,
    ) acquires ClearingCycle { }

    /// Settle netted positions via settlement module
    public entry fun settle_cycle(
        authority: &signer,
        cycle_id: u64,
    ) acquires ClearingCycle { }

    #[view]
    public fun netting_efficiency(cycle_id: u64): u64 acquires ClearingCycle { }
}
```

### 4.2 Netting Invariant

| ID | Invariant |
|----|-----------|
| CLR-1 | Sum of all net positions in a cycle MUST equal zero |
| CLR-2 | Net settlement ≤ N-1 transfers for N participants (spanning tree bound) |
| CLR-3 | Netting MUST NOT alter total value — only reduce transfer count |

---

## 5. Bridge Module

### 5.1 Design

Lock-and-mint bridge for cross-chain asset movement. Secured by multi-sig
threshold of authorized bridge operators (institutional partners).

```move
module 0x1::bridge {
    use std::string::String;

    const CHAIN_ETHEREUM: u8 = 1;
    const CHAIN_BSC: u8 = 2;
    const CHAIN_SOLANA: u8 = 3;
    const CHAIN_BITCOIN: u8 = 4;

    const STATUS_INITIATED: u8 = 0;
    const STATUS_CONFIRMED: u8 = 1;
    const STATUS_COMPLETED: u8 = 2;
    const STATUS_REFUNDED: u8 = 3;

    /// Bridge configuration
    struct BridgeConfig has key {
        /// Required confirmations from operators
        threshold: u64,
        /// Authorized bridge operators
        operators: vector<address>,
        /// Supported chains
        supported_chains: vector<u8>,
        /// Fee in basis points
        fee_bps: u64,
        /// Paused flag
        paused: bool,
    }

    /// Inbound bridge request (external → CreditChain)
    struct InboundRequest has key, store {
        request_id: u64,
        source_chain: u8,
        source_tx_hash: vector<u8>,
        recipient: address,
        asset: String,
        amount: u64,
        confirmations: vector<address>,  // operators who confirmed
        status: u8,
    }

    /// Outbound bridge request (CreditChain → external)
    struct OutboundRequest has key, store {
        request_id: u64,
        destination_chain: u8,
        destination_address: vector<u8>,
        sender: address,
        asset: String,
        amount: u64,
        status: u8,
    }

    /// Initiate outbound bridge transfer (lock on CreditChain)
    public entry fun bridge_out(
        sender: &signer,
        destination_chain: u8,
        destination_address: vector<u8>,
        asset: String,
        amount: u64,
    ) acquires BridgeConfig { }

    /// Confirm inbound bridge transfer (operator attestation)
    public entry fun confirm_inbound(
        operator: &signer,
        request_id: u64,
    ) acquires BridgeConfig, InboundRequest { }

    /// Complete inbound bridge (mint wrapped asset after threshold)
    public entry fun complete_inbound(
        authority: &signer,
        request_id: u64,
    ) acquires InboundRequest { }

    /// Refund expired outbound request
    public entry fun refund_outbound(
        authority: &signer,
        request_id: u64,
    ) acquires OutboundRequest { }
}
```

### 5.2 Bridge Invariants

| ID | Invariant |
|----|-----------|
| BRG-1 | Inbound completion requires ≥ threshold operator confirmations |
| BRG-2 | Outbound MUST lock assets before emitting bridge event |
| BRG-3 | Refund path MUST exist for every outbound after timeout |
| BRG-4 | Bridge MUST be pausable for emergency |
| BRG-5 | Fee MUST be collected before asset delivery |

---

## 6. WorldLine Anchoring Module

### 6.1 Design

Anchors real-world events (trade executions, regulatory filings, audit checkpoints)
to immutable on-chain timestamps. Provides tamper-proof audit trail for institutions.

```move
module 0x1::worldline {
    use std::string::String;

    /// Anchor types
    const ANCHOR_TRADE: u8 = 1;
    const ANCHOR_REGULATORY: u8 = 2;
    const ANCHOR_AUDIT: u8 = 3;
    const ANCHOR_SETTLEMENT: u8 = 4;
    const ANCHOR_COMPLIANCE: u8 = 5;

    /// An immutable anchor point
    struct Anchor has key, store, drop {
        anchor_id: u64,
        anchor_type: u8,
        /// SHA-256 hash of off-chain data
        data_hash: vector<u8>,
        /// Human-readable description
        description: String,
        /// Institution that created this anchor
        institution: address,
        /// Epoch and timestamp
        epoch: u64,
        timestamp: u64,
    }

    /// Authorized institutions for anchoring
    struct AnchorRegistry has key {
        authorized_institutions: vector<address>,
        total_anchors: u64,
    }

    /// Create an anchor — immutable once written
    public entry fun anchor(
        institution: &signer,
        anchor_type: u8,
        data_hash: vector<u8>,
        description: String,
    ) acquires AnchorRegistry { }

    /// Verify an anchor exists with given hash
    #[view]
    public fun verify_anchor(anchor_id: u64, expected_hash: vector<u8>): bool
        acquires Anchor { }

    /// Get total anchor count
    #[view]
    public fun total_anchors(): u64 acquires AnchorRegistry { }
}
```

---

## 7. Compliance Module

### 7.1 Design

On-chain KYC/AML attestation registry. Institutions attest to customer
verification status without revealing PII on-chain.

```move
module 0x1::compliance {
    use std::string::String;

    /// KYC verification levels
    const KYC_NONE: u8 = 0;
    const KYC_BASIC: u8 = 1;       // Name + email
    const KYC_STANDARD: u8 = 2;    // + ID document
    const KYC_ENHANCED: u8 = 3;    // + proof of address + source of funds
    const KYC_INSTITUTIONAL: u8 = 4; // Full institutional due diligence

    /// KYC attestation — no PII stored on-chain
    struct KYCAttestation has key, store {
        subject: address,
        level: u8,
        /// Attesting institution
        attestor: address,
        /// Jurisdiction (ISO 3166-1 alpha-2)
        jurisdiction: String,
        /// Expiry timestamp
        valid_until: u64,
        /// Hash of off-chain KYC record
        record_hash: vector<u8>,
    }

    /// Authorized KYC attestors (regulated institutions)
    struct AttestorRegistry has key {
        authorized_attestors: vector<address>,
    }

    /// Attest KYC level for an address
    public entry fun attest(
        attestor: &signer,
        subject: address,
        level: u8,
        jurisdiction: String,
        valid_until: u64,
        record_hash: vector<u8>,
    ) acquires AttestorRegistry { }

    /// Check if address has minimum KYC level
    #[view]
    public fun has_kyc_level(subject: address, min_level: u8): bool
        acquires KYCAttestation { }

    /// Revoke attestation
    public entry fun revoke(
        attestor: &signer,
        subject: address,
    ) acquires KYCAttestation { }
}
```

---

## 8. Oracle Module

### 8.1 Design

Aggregated price oracle for CCC, IUSD, BTC, ETH, and other assets.
Used by settlement, bridge, and clearing modules for fair-value pricing.

```move
module 0x1::oracle {
    use std::string::String;

    struct PriceFeed has key, store {
        asset: String,
        price_usd: u64,        // 8 decimal places
        confidence_bps: u64,   // confidence interval in bps
        sources_count: u64,    // number of oracle sources
        last_updated: u64,
        staleness_threshold: u64,
    }

    struct OracleConfig has key {
        authorized_feeders: vector<address>,
        min_sources: u64,       // minimum sources for valid price
    }

    /// Submit price data point
    public entry fun submit_price(
        feeder: &signer,
        asset: String,
        price_usd: u64,
    ) acquires OracleConfig, PriceFeed { }

    /// Get latest price (reverts if stale)
    #[view]
    public fun get_price(asset: String): (u64, u64)
        acquires PriceFeed { }

    /// Check if price is fresh
    #[view]
    public fun is_fresh(asset: String): bool
        acquires PriceFeed { }
}
```

---

## 9. AI Agent Registry Module

### 9.1 Design

Registers AI agents operating on CreditChain, records execution proofs,
and manages agent permissions for autonomous financial operations.

```move
module 0x1::agent_registry {
    use std::string::String;

    /// Agent capability levels
    const AGENT_READONLY: u8 = 0;
    const AGENT_TRADE: u8 = 1;
    const AGENT_SETTLE: u8 = 2;
    const AGENT_FULL: u8 = 3;

    struct Agent has key, store {
        agent_id: u64,
        owner: address,
        agent_address: address,
        name: String,
        capability_level: u8,
        /// Hash of agent code/model
        code_hash: vector<u8>,
        /// Spending limit per epoch (in IUSD)
        epoch_limit: u64,
        epoch_spent: u64,
        active: bool,
    }

    /// Register an AI agent
    public entry fun register_agent(
        owner: &signer,
        agent_address: address,
        name: String,
        capability_level: u8,
        code_hash: vector<u8>,
        epoch_limit: u64,
    ) { }

    /// Record agent execution proof
    public entry fun record_execution(
        agent: &signer,
        action_hash: vector<u8>,
        result_hash: vector<u8>,
    ) acquires Agent { }

    /// Deactivate agent (owner only)
    public entry fun deactivate(
        owner: &signer,
        agent_id: u64,
    ) acquires Agent { }
}
```

---

## 10. Deployment Model: Public vs. Private

### 10.1 Deployment Configurations

CreditChain supports multiple deployment models based on institutional requirements:

| Model | Validators | Permissioning | Use Case |
|-------|-----------|---------------|----------|
| **Public Mainnet** | Open staking (50-500) | Permissionless reads, permissioned writes | Global settlement |
| **Consortium Chain** | Invited institutions (7-21) | Fully permissioned | Inter-bank settlement |
| **Private Enterprise** | Single organization (4-7) | Fully private | Internal settlement |
| **Hybrid** | Core validators + public full nodes | Layered access | Institutional + retail |

### 10.2 Privacy Controls

```yaml
# creditchain-node.yaml — Privacy configuration
access_control:
  # Who can submit transactions
  transaction_submission: "permissioned"  # or "open"
  # Who can read state
  state_read: "permissioned"  # or "open"
  # Who can run full nodes
  full_node_join: "permissioned"  # or "open"
  # Authorized submitter list (for permissioned mode)
  authorized_submitters:
    - "0x..."
    - "0x..."
```

### 10.3 Enterprise Features

| Feature | Public | Consortium | Private |
|---------|--------|-----------|---------|
| KYC required for transactions | Optional | Required | Internal |
| Validator identity known | No | Yes | Yes |
| Transaction privacy | Pseudonymous | Confidential | Full privacy |
| Regulatory reporting | Self-service | Built-in | Custom |
| SLA guarantees | Best-effort | 99.9% | 99.99% |
| Geographic restrictions | None | Configurable | Full control |

---

## 11. Module Dependency Graph

```
                    creditchain_framework
                           │
              ┌────────────┼────────────┐
              │            │            │
           oracle      compliance    worldline
              │            │            │
              ├────────────┤            │
              │            │            │
            iusd      credit_score      │
              │            │            │
              ├────────────┘            │
              │                         │
          settlement ───────────────────┘
              │
          clearing
              │
          bridge
              │
        agent_registry
```

---

## 12. Module Invariants (Master List)

| Module | ID | Invariant |
|--------|----|-----------|
| IUSD | IUSD-1 | total_supply = sum of all balances |
| IUSD | IUSD-2 | reserve_ratio ≥ 100% for minting |
| Settlement | SET-1 | DvP is atomic (both legs or neither) |
| Settlement | SET-4 | Batch net positions sum to zero |
| Clearing | CLR-1 | Net positions sum to zero |
| Clearing | CLR-2 | Net transfers ≤ N-1 (spanning tree) |
| Bridge | BRG-1 | Threshold confirmations required |
| Bridge | BRG-3 | Refund path always exists |
| Oracle | ORC-1 | Price requires min_sources attestations |
| Oracle | ORC-2 | Stale prices MUST revert |
| Compliance | CMP-1 | No PII stored on-chain |
| Compliance | CMP-2 | Only authorized attestors can attest |
| WorldLine | WL-1 | Anchors are immutable once written |
| Agent | AGT-1 | Agent spending ≤ epoch_limit |

---

*CreditChain Move Modules — Institution-Grade Financial Primitives on Chain*
