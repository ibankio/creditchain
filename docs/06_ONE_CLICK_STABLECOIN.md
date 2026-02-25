# One-Click Stablecoin Platform: Stablecoin-as-a-Service on CreditChain

> Document 06 | CreditChain Design Series | Version 1.0
> Scope: StablecoinFactory, StablecoinRegistry, cross-stablecoin operations, OpenIBank integration
> Companion to: OpenIBank Doc 15 (IUSD Spec), Doc 25 (Enterprise Whitelabel), Doc 48 (Tokenomics)

---

## 1. Executive Summary

CreditChain transforms from hosting a single stablecoin (IUSD) to operating a
**Stablecoin-as-a-Service** platform where any institution can create their own
branded stablecoin in a single transaction. Powered by Move's Fungible Asset V2
object model, the StablecoinFactory enables unlimited runtime asset creation
without deploying new smart contracts.

### The One-Click Promise

```
Institution calls ONE function → branded stablecoin exists on CreditChain
                                  ├── Fully operational mint/burn
                                  ├── Reserve attestation pipeline
                                  ├── Rate limiting & circuit breakers
                                  ├── Compliance-gated operations
                                  ├── Cross-chain bridge ready
                                  ├── Interoperable with ALL other stablecoins
                                  └── Dashboard & API access
```

### Native Genesis Stablecoins (The Big 6)

CreditChain launches with **6 native institutional stablecoins** covering the
world's 6 most popular currencies by forex trading volume — deployed at genesis,
operated by CreditChain Foundation, instantly available for global settlement:

| Symbol | Name | Peg | ISO | Decimals | Role |
|--------|------|-----|-----|----------|------|
| **IUSD** | iBank US Dollar | USD | 840 | 6 | Primary settlement currency |
| **IEUR** | iBank Euro | EUR | 978 | 6 | Eurozone settlement |
| **IJPY** | iBank Japanese Yen | JPY | 392 | 0 | Asia-Pacific settlement |
| **IGBP** | iBank British Pound | GBP | 826 | 6 | UK & Commonwealth settlement |
| **ICNY** | iBank Chinese Yuan | CNY | 156 | 6 | Greater China settlement |
| **ICAD** | iBank Canadian Dollar | CAD | 124 | 6 | North American settlement |

These 6 currencies cover **~88% of global forex trading volume** and represent
the major financial centers: New York, Frankfurt, Tokyo, London, Shanghai, Toronto.

```
Global Settlement Coverage at Genesis:
┌──────────────────────────────────────────────────────────────────┐
│  IUSD ──── Americas, global reserve     (USD, ~44% forex vol)    │
│  IEUR ──── Eurozone, EU settlement      (EUR, ~16% forex vol)    │
│  IJPY ──── Japan, Asia-Pacific          (JPY, ~8.3% forex vol)   │
│  IGBP ──── UK, Commonwealth             (GBP, ~6.5% forex vol)   │
│  ICNY ──── Greater China, Belt & Road   (CNY, ~7.0% forex vol)   │
│  ICAD ──── Canada, commodity trade       (CAD, ~5.2% forex vol)  │
│                                                                  │
│  Combined: ~88% of global forex volume covered from day one      │
│                                                                  │
│  + Unlimited user-created stablecoins via factory                │
│  + Cross-currency atomic swaps via oracle-priced forex           │
│  + Multilateral netting across ALL currencies                    │
│  + 15 native cross-pairs (C(6,2)) for instant forex settlement   │
└──────────────────────────────────────────────────────────────────┘
```

### Additional User-Created Stablecoins (Examples)

| Symbol | Name | Peg | Issuer | Use Case |
|--------|------|-----|--------|----------|
| MUSD | Morgan Stable Dollar | USD | JPMorgan consortium | Institutional USD settlement |
| SGDS | Singapore Dollar Stable | SGD | MAS-authorized bank | APAC settlement |
| CBDC-X | Central Bank Digital X | XDR | Central Bank | Sovereign CBDC |
| GOLD | Gold-Backed Token | XAU | Commodity vault | Commodity settlement |
| CORP-A | Acme Internal Dollar | USD | Acme Corp | Internal treasury |

---

## 2. Architecture: Why Fungible Asset V2

### 2.1 The Problem with Old Coin<T> Framework

The original Move Coin framework requires a **static type parameter** for each coin:

```move
// Old pattern: requires NEW MODULE per coin
module 0x1::iusd {
    struct IUSD has key, store, drop {}
    // MintCapability<IUSD>, BurnCapability<IUSD> — type-bound
}
module 0x1::musd {
    struct MUSD has key, store, drop {}
    // Requires separate module deployment per coin
}
```

This means creating a new stablecoin requires:
1. Writing a new Move module
2. Compiling it
3. Publishing it to the chain via governance proposal
4. Waiting for governance approval

**This is NOT one-click. This is weeks of work.**

### 2.2 Fungible Asset V2: Runtime Asset Creation

FA V2 uses the **Object model** — each asset is an Object with a unique address,
created at runtime without new module deployment:

```move
// New pattern: ONE factory module creates UNLIMITED coins at runtime
module 0x1::stablecoin_factory {
    public entry fun create_stablecoin(
        creator: &signer,
        name: String,
        symbol: String,
        decimals: u8,
        peg_currency: String,
        // ... config
    ) {
        // Creates a new Object with deterministic address
        let constructor_ref = object::create_named_object(creator, seed);

        // Attaches fungible asset capabilities to the Object
        primary_fungible_store::create_primary_store_enabled_fungible_asset(
            &constructor_ref,
            option::some(max_supply),
            name, symbol, decimals,
            icon_uri, project_uri,
        );

        // Generate and store control references
        let mint_ref = fungible_asset::generate_mint_ref(&constructor_ref);
        let burn_ref = fungible_asset::generate_burn_ref(&constructor_ref);
        let transfer_ref = fungible_asset::generate_transfer_ref(&constructor_ref);

        // Store on the object (only factory can access)
        move_to(&object_signer, ManagedRefs { mint_ref, burn_ref, transfer_ref });
        move_to(&object_signer, StablecoinConfig { ... });
        move_to(&object_signer, StablecoinGovernance { ... });
        move_to(&object_signer, ReserveAttestation { ... });
        move_to(&object_signer, RateTracker { ... });

        // Register in global registry
        stablecoin_registry::register(metadata_addr, symbol);
    }
}
```

### 2.3 Decision Matrix

| Aspect | Old Coin&lt;T&gt; | FA V2 (Object) | Winner |
|--------|-------------|-----------------|--------|
| New coin creation | New module required | Runtime creation | **FA V2** |
| Deployment speed | Days/weeks (governance) | **One transaction** | **FA V2** |
| Scalability | Module per coin | Unlimited from factory | **FA V2** |
| Store management | Manual `register<T>()` per user | Automatic primary store | **FA V2** |
| Capability model | Type-bound (static) | Object-bound (dynamic) | **FA V2** |
| Supply tracking | Optional, per-type | Concurrent aggregator v2 | **FA V2** |
| Cross-coin operations | Type gymnastics | Uniform `FungibleAsset` | **FA V2** |
| Bridge compatibility | Per-type adapter | Single adapter for all | **FA V2** |

---

## 3. On-Chain Architecture

### 3.1 Module Hierarchy

```
0x1::stablecoin_factory    ← Creates stablecoins (ONE factory for ALL)
0x1::stablecoin_registry   ← Global registry (discovery, duplicate prevention)
0x1::stablecoin_compliance ← KYC/AML gates for creation and operations
0x1::stablecoin_bridge     ← Cross-chain bridge adapter for any stablecoin
0x1::stablecoin_swap       ← Atomic cross-stablecoin swaps

// Inherited from CreditChain framework:
0x1::fungible_asset        ← FA V2 core (mint, burn, transfer, deposit, withdraw)
0x1::primary_fungible_store ← Auto-created stores per account
0x1::object                ← Object creation and management
0x1::settlement            ← DvP atomic settlement
0x1::worldline             ← Immutable audit anchoring
0x1::compliance            ← KYC attestation registry
0x1::oracle                ← Price feeds for cross-stablecoin rates
```

### 3.2 Object Model Per Stablecoin

Each stablecoin created by the factory is a **Move Object** with the following
resources stored at the object's address:

```
Object @ 0xSTABLECOIN_ADDR
├── Metadata (name, symbol, decimals, supply — from FA V2)
├── ManagedRefs
│   ├── mint_ref: MintRef
│   ├── burn_ref: BurnRef
│   └── transfer_ref: TransferRef
├── StablecoinConfig
│   ├── peg_currency: String          // "USD", "EUR", "GBP", "SGD", "XAU"
│   ├── peg_decimals: u8             // Peg currency decimal places
│   ├── max_supply: u64              // 0 = unlimited
│   ├── min_reserve_ratio_bps: u64   // Default 10000 (100%)
│   ├── max_attestation_age_secs: u64 // Default 3600 (1 hour)
│   ├── platform_fee_mint_bps: u64   // Default 5 (0.05%)
│   ├── platform_fee_burn_bps: u64   // Default 5 (0.05%)
│   ├── creator: address
│   └── created_at: u64
├── StablecoinGovernance
│   ├── admin: address                // Can update config
│   ├── issuers: vector<address>      // Can mint/burn
│   ├── auditors: vector<address>     // Can attest reserves
│   ├── paused: bool                  // Emergency circuit breaker
│   └── frozen_accounts: vector<address> // Compliance freeze
├── ReserveAttestation
│   ├── total_reserves_value: u64     // In peg currency units
│   ├── reserve_ratio_bps: u64       // Basis points (10000 = 100%)
│   ├── proof_hash: vector<u8>       // SHA-256 of off-chain proof
│   ├── auditor: address             // Who attested
│   ├── attested_at: u64             // Timestamp
│   └── attestation_count: u64
└── RateTracker
    ├── epoch_mint_volume: u64
    ├── epoch_burn_volume: u64
    ├── epoch_mint_limit: u64         // Per-epoch cap
    ├── epoch_burn_limit: u64
    ├── single_mint_limit: u64        // Per-transaction cap
    ├── single_burn_limit: u64
    └── current_epoch: u64
```

### 3.3 StablecoinFactory Module

```move
module creditchain_framework::stablecoin_factory {
    use std::signer;
    use std::string::String;
    use std::option;
    use creditchain_framework::object::{Self, Object, ConstructorRef};
    use creditchain_framework::fungible_asset::{Self, MintRef, BurnRef, TransferRef, Metadata};
    use creditchain_framework::primary_fungible_store;
    use creditchain_framework::stablecoin_registry;
    use creditchain_framework::compliance;
    use creditchain_framework::worldline;
    use creditchain_framework::event;
    use creditchain_framework::timestamp;

    // ─── Error Codes ────────────────────────────────────────────

    const E_NOT_AUTHORIZED: u64 = 1;
    const E_SYMBOL_TAKEN: u64 = 2;
    const E_PAUSED: u64 = 3;
    const E_STALE_ATTESTATION: u64 = 4;
    const E_INSUFFICIENT_RESERVES: u64 = 5;
    const E_RATE_LIMIT_EXCEEDED: u64 = 6;
    const E_INSUFFICIENT_BALANCE: u64 = 7;
    const E_NOT_ISSUER: u64 = 8;
    const E_NOT_AUDITOR: u64 = 9;
    const E_NOT_ADMIN: u64 = 10;
    const E_KYC_REQUIRED: u64 = 11;
    const E_ACCOUNT_FROZEN: u64 = 12;
    const E_INVALID_DECIMALS: u64 = 13;
    const E_SYMBOL_TOO_LONG: u64 = 14;
    const E_SINGLE_TX_LIMIT: u64 = 15;

    // ─── Stored Resources (on each stablecoin Object) ───────────

    struct ManagedRefs has key {
        mint_ref: MintRef,
        burn_ref: BurnRef,
        transfer_ref: TransferRef,
    }

    struct StablecoinConfig has key {
        peg_currency: String,
        peg_decimals: u8,
        max_supply: u64,
        min_reserve_ratio_bps: u64,
        max_attestation_age_secs: u64,
        platform_fee_mint_bps: u64,
        platform_fee_burn_bps: u64,
        creator: address,
        created_at: u64,
    }

    struct StablecoinGovernance has key {
        admin: address,
        issuers: vector<address>,
        auditors: vector<address>,
        paused: bool,
        frozen_accounts: vector<address>,
    }

    struct ReserveAttestation has key {
        total_reserves_value: u64,
        reserve_ratio_bps: u64,
        proof_hash: vector<u8>,
        auditor: address,
        attested_at: u64,
        attestation_count: u64,
    }

    struct RateTracker has key {
        epoch_mint_volume: u64,
        epoch_burn_volume: u64,
        epoch_mint_limit: u64,
        epoch_burn_limit: u64,
        single_mint_limit: u64,
        single_burn_limit: u64,
        current_epoch: u64,
    }

    // ─── Events ─────────────────────────────────────────────────

    #[event]
    struct StablecoinCreatedEvent has drop, store {
        metadata_address: address,
        name: String,
        symbol: String,
        decimals: u8,
        peg_currency: String,
        creator: address,
    }

    #[event]
    struct MintEvent has drop, store {
        metadata_address: address,
        issuer: address,
        recipient: address,
        amount: u64,
        new_total_supply: u64,
        purpose: String,
    }

    #[event]
    struct BurnEvent has drop, store {
        metadata_address: address,
        issuer: address,
        from: address,
        amount: u64,
        new_total_supply: u64,
        purpose: String,
    }

    #[event]
    struct AttestationEvent has drop, store {
        metadata_address: address,
        auditor: address,
        total_reserves_value: u64,
        reserve_ratio_bps: u64,
        proof_hash: vector<u8>,
    }

    #[event]
    struct PauseEvent has drop, store {
        metadata_address: address,
        admin: address,
        paused: bool,
    }

    // ─── Entry Functions ────────────────────────────────────────

    /// ONE-CLICK STABLECOIN CREATION
    /// Creates a fully operational stablecoin in a single transaction.
    ///
    /// Requirements:
    ///   - Creator has KYC_INSTITUTIONAL (level 4) attestation
    ///   - Symbol not already taken in registry
    ///   - Symbol length 2-10 characters, uppercase
    ///   - Decimals 0-18
    ///   - Creation fee paid in CCC
    ///
    /// After this call, the stablecoin is:
    ///   - Registered in StablecoinRegistry
    ///   - Ready for reserve attestation
    ///   - Ready for minting (after attestation)
    ///   - Bridge-compatible
    ///   - Settlement-compatible
    ///   - Dashboard-visible
    public entry fun create_stablecoin(
        creator: &signer,
        name: String,
        symbol: String,
        decimals: u8,
        peg_currency: String,
        peg_decimals: u8,
        max_supply: u64,                  // 0 = unlimited
        min_reserve_ratio_bps: u64,       // 10000 = 100%
        max_attestation_age_secs: u64,    // 3600 = 1 hour
        epoch_mint_limit: u64,
        epoch_burn_limit: u64,
        single_mint_limit: u64,
        single_burn_limit: u64,
        icon_uri: String,
        project_uri: String,
    ) acquires ... { /* implementation in P9 prompt */ }

    /// Mint stablecoin tokens to a recipient.
    ///
    /// Requirements:
    ///   - Caller is authorized issuer for this stablecoin
    ///   - Stablecoin is not paused
    ///   - Reserve attestation is fresh (< max_attestation_age_secs)
    ///   - Reserve ratio would remain >= min_reserve_ratio_bps after mint
    ///   - Amount within rate limits (single tx + epoch)
    ///   - Recipient is not frozen
    public entry fun mint(
        issuer: &signer,
        metadata_addr: address,
        recipient: address,
        amount: u64,
        purpose: String,
    ) acquires ManagedRefs, StablecoinConfig, StablecoinGovernance,
              ReserveAttestation, RateTracker { /* ... */ }

    /// Burn stablecoin tokens from an account.
    ///
    /// Requirements:
    ///   - Caller is authorized issuer for this stablecoin
    ///   - Stablecoin is not paused
    ///   - Account has sufficient balance
    ///   - Amount within rate limits
    ///   - Account is not frozen
    public entry fun burn(
        issuer: &signer,
        metadata_addr: address,
        from: address,
        amount: u64,
        purpose: String,
    ) acquires ManagedRefs, StablecoinGovernance, RateTracker { /* ... */ }

    /// Submit reserve attestation for a stablecoin.
    ///
    /// Requirements:
    ///   - Caller is authorized auditor for this stablecoin
    ///   - Stablecoin is not paused
    public entry fun attest_reserves(
        auditor: &signer,
        metadata_addr: address,
        total_reserves_value: u64,
        proof_hash: vector<u8>,
    ) acquires StablecoinGovernance, ReserveAttestation { /* ... */ }

    /// Emergency pause/unpause all operations.
    public entry fun set_paused(
        admin: &signer,
        metadata_addr: address,
        paused: bool,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Add an authorized issuer.
    public entry fun add_issuer(
        admin: &signer,
        metadata_addr: address,
        new_issuer: address,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Remove an authorized issuer.
    public entry fun remove_issuer(
        admin: &signer,
        metadata_addr: address,
        issuer: address,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Add an authorized auditor.
    public entry fun add_auditor(
        admin: &signer,
        metadata_addr: address,
        new_auditor: address,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Freeze an account (compliance action).
    public entry fun freeze_account(
        admin: &signer,
        metadata_addr: address,
        account: address,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Unfreeze an account.
    public entry fun unfreeze_account(
        admin: &signer,
        metadata_addr: address,
        account: address,
    ) acquires StablecoinGovernance { /* ... */ }

    /// Update rate limits.
    public entry fun update_rate_limits(
        admin: &signer,
        metadata_addr: address,
        epoch_mint_limit: u64,
        epoch_burn_limit: u64,
        single_mint_limit: u64,
        single_burn_limit: u64,
    ) acquires StablecoinGovernance, RateTracker { /* ... */ }

    /// Transfer admin role.
    public entry fun transfer_admin(
        admin: &signer,
        metadata_addr: address,
        new_admin: address,
    ) acquires StablecoinGovernance { /* ... */ }

    // ─── View Functions ─────────────────────────────────────────

    #[view]
    public fun total_supply(metadata_addr: address): u64 { /* ... */ }

    #[view]
    public fun reserve_ratio_bps(metadata_addr: address): u64
        acquires ReserveAttestation { /* ... */ }

    #[view]
    public fun is_paused(metadata_addr: address): bool
        acquires StablecoinGovernance { /* ... */ }

    #[view]
    public fun attestation_age_secs(metadata_addr: address): u64
        acquires ReserveAttestation { /* ... */ }

    #[view]
    public fun get_config(metadata_addr: address): (String, String, u8, String, u64)
        acquires StablecoinConfig { /* ... */ }

    #[view]
    public fun get_issuers(metadata_addr: address): vector<address>
        acquires StablecoinGovernance { /* ... */ }

    #[view]
    public fun epoch_mint_remaining(metadata_addr: address): u64
        acquires RateTracker { /* ... */ }
}
```

### 3.4 StablecoinRegistry Module

```move
module creditchain_framework::stablecoin_registry {
    use std::string::String;
    use std::vector;
    use creditchain_framework::smart_table::{Self, SmartTable};

    /// Global registry of all stablecoins on CreditChain.
    struct StablecoinRegistry has key {
        /// Symbol → metadata object address
        symbol_to_address: SmartTable<String, address>,
        /// All metadata addresses in creation order
        all_stablecoins: vector<address>,
        /// Total count
        total_count: u64,
    }

    /// Per-stablecoin public info (queryable by anyone).
    struct StablecoinInfo has store, drop, copy {
        metadata_address: address,
        name: String,
        symbol: String,
        decimals: u8,
        peg_currency: String,
        creator: address,
        created_at: u64,
    }

    /// Initialize registry at genesis (called once by framework).
    public fun initialize(framework: &signer) { /* ... */ }

    /// Register a new stablecoin (called by factory).
    public(friend) fun register(
        metadata_addr: address,
        symbol: String,
        name: String,
        decimals: u8,
        peg_currency: String,
        creator: address,
    ) acquires StablecoinRegistry { /* ... */ }

    /// Check if symbol is taken.
    #[view]
    public fun symbol_exists(symbol: String): bool
        acquires StablecoinRegistry { /* ... */ }

    /// Get metadata address by symbol.
    #[view]
    public fun get_by_symbol(symbol: String): address
        acquires StablecoinRegistry { /* ... */ }

    /// Get total stablecoin count.
    #[view]
    public fun total_count(): u64
        acquires StablecoinRegistry { /* ... */ }

    /// List all stablecoins (paginated).
    #[view]
    public fun list(offset: u64, limit: u64): vector<StablecoinInfo>
        acquires StablecoinRegistry { /* ... */ }

    /// List stablecoins by peg currency.
    #[view]
    public fun list_by_peg(peg_currency: String): vector<StablecoinInfo>
        acquires StablecoinRegistry { /* ... */ }
}
```

---

## 4. Cross-Stablecoin Operations

### 4.1 Atomic Swap Module

```move
module creditchain_framework::stablecoin_swap {
    use creditchain_framework::fungible_asset::{Self, FungibleAsset};
    use creditchain_framework::stablecoin_factory;
    use creditchain_framework::oracle;

    /// Atomic swap between any two stablecoins.
    ///
    /// Flow:
    ///   1. Get oracle price for from_peg/to_peg
    ///   2. Calculate to_amount = from_amount * oracle_rate
    ///   3. Withdraw from_coin from sender
    ///   4. Deposit to_coin to sender (from liquidity pool)
    ///   5. Deposit from_coin to liquidity pool
    ///   All in single transaction (atomic).
    public entry fun swap(
        sender: &signer,
        from_metadata: address,    // e.g., IUSD metadata address
        to_metadata: address,      // e.g., EUSD metadata address
        from_amount: u64,
        min_to_amount: u64,        // Slippage protection
    ) { /* ... */ }

    /// Direct swap between two stablecoins with the same peg (1:1).
    /// No oracle needed — same peg currency means same value.
    /// Example: IUSD ↔ MUSD (both USD-pegged) at 1:1
    public entry fun swap_same_peg(
        sender: &signer,
        from_metadata: address,
        to_metadata: address,
        amount: u64,
    ) { /* ... */ }
}
```

### 4.2 Cross-Stablecoin Clearing (Native Multi-Currency)

```
Institution A (New York):  owes Institution B  100,000 IEUR
Institution B (London):    owes Institution A   80,000 IUSD
Institution C (Toronto):   owes Institution A   50,000 IGBP
Institution D (Tokyo):     owes Institution B  15,000,000 IJPY
Institution E (Shanghai):  owes Institution A  200,000 ICNY

Multilateral Netting (via 0x1::clearing):
  1. Convert all obligations to common unit (USD via oracle)
     A owes B: 100,000 IEUR = ~108,500 USD
     B owes A:  80,000 IUSD =  80,000 USD
     C owes A:  50,000 IGBP = ~63,200 USD
     D owes B: 15,000,000 IJPY = ~100,000 USD
     E owes A: 200,000 ICNY = ~27,400 USD
  2. Calculate net positions (in USD equivalent):
     A: net payable  28,500 USD
     B: net receivable 54,100 USD
     C: net payable   63,200 USD
     D: net payable   25,600 USD (but... net positions must sum to 0, settled against B)
  3. Settle net amounts via DvP (0x1::settlement)
     — Each net payment settles in the receiver's preferred native stablecoin
     — Oracle provides real-time forex rates for conversion
  4. All in < 1 second, fully atomic
  4. Each institution's stablecoin supply adjusted accordingly
```

### 4.3 Bridge Adapter

Any factory-created stablecoin automatically bridges to external chains:

```
CreditChain                    Ethereum
┌──────────────┐              ┌──────────────┐
│ MUSD (FA V2) │  Bridge      │ wMUSD        │
│ burn(amount) │─────────────►│ ERC-20 mint  │
│              │  Operators   │              │
│ mint(amount) │◄─────────────│ ERC-20 burn  │
└──────────────┘              └──────────────┘
```

The bridge module queries StablecoinRegistry to resolve any symbol,
then uses the factory's ManagedRefs to mint/burn. **No per-coin bridge
configuration needed.**

---

## 5. Native Genesis Stablecoins (The Big 6)

### 5.1 Genesis Initialization

All 6 native stablecoins are created at genesis by the CreditChain Framework
signer. They are the first 6 entries in StablecoinRegistry — immutable,
foundation-operated, and instantly available.

```move
/// Called once at genesis. Creates the 6 native institutional stablecoins.
public fun initialize_genesis_stablecoins(framework: &signer) {

    // ── 1. IUSD — iBank US Dollar ─────────────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank US Dollar"),
        string::utf8(b"IUSD"),
        6,                                    // decimals
        string::utf8(b"USD"),                 // peg: US Dollar
        2,                                    // peg decimals
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        10000000000000,                       // epoch_mint_limit: 10M
        10000000000000,                       // epoch_burn_limit: 10M
        1000000000000,                        // single_mint_limit: 1M
        1000000000000,                        // single_burn_limit: 1M
        string::utf8(b"https://creditchain.org/assets/iusd-icon.png"),
        string::utf8(b"https://iusd.creditchain.org"),
    );

    // ── 2. IEUR — iBank Euro ──────────────────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank Euro"),
        string::utf8(b"IEUR"),
        6,                                    // decimals
        string::utf8(b"EUR"),                 // peg: Euro
        2,                                    // peg decimals
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        10000000000000,                       // epoch_mint_limit: 10M
        10000000000000,                       // epoch_burn_limit: 10M
        1000000000000,                        // single_mint_limit: 1M
        1000000000000,                        // single_burn_limit: 1M
        string::utf8(b"https://creditchain.org/assets/ieur-icon.png"),
        string::utf8(b"https://ieur.creditchain.org"),
    );

    // ── 3. IJPY — iBank Japanese Yen ──────────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank Japanese Yen"),
        string::utf8(b"IJPY"),
        0,                                    // decimals: JPY has NO minor units (ISO 4217)
        string::utf8(b"JPY"),                 // peg: Japanese Yen
        0,                                    // peg decimals: 0 for JPY
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        1500000000000,                        // epoch_mint_limit: 1.5B JPY (~10M USD)
        1500000000000,                        // epoch_burn_limit: 1.5B JPY
        150000000000,                         // single_mint_limit: 150M JPY (~1M USD)
        150000000000,                         // single_burn_limit: 150M JPY
        string::utf8(b"https://creditchain.org/assets/ijpy-icon.png"),
        string::utf8(b"https://ijpy.creditchain.org"),
    );

    // ── 4. IGBP — iBank British Pound ─────────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank British Pound"),
        string::utf8(b"IGBP"),
        6,                                    // decimals
        string::utf8(b"GBP"),                 // peg: British Pound
        2,                                    // peg decimals
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        10000000000000,                       // epoch_mint_limit: 10M
        10000000000000,                       // epoch_burn_limit: 10M
        1000000000000,                        // single_mint_limit: 1M
        1000000000000,                        // single_burn_limit: 1M
        string::utf8(b"https://creditchain.org/assets/igbp-icon.png"),
        string::utf8(b"https://igbp.creditchain.org"),
    );

    // ── 5. ICNY — iBank Chinese Yuan ────────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank Chinese Yuan"),
        string::utf8(b"ICNY"),
        6,                                    // decimals
        string::utf8(b"CNY"),                 // peg: Chinese Yuan
        2,                                    // peg decimals
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        10000000000000,                       // epoch_mint_limit: 10M
        10000000000000,                       // epoch_burn_limit: 10M
        1000000000000,                        // single_mint_limit: 1M
        1000000000000,                        // single_burn_limit: 1M
        string::utf8(b"https://creditchain.org/assets/icny-icon.png"),
        string::utf8(b"https://icny.creditchain.org"),
    );

    // ── 6. ICAD — iBank Canadian Dollar ─────────────────────
    stablecoin_factory::create_stablecoin(
        framework,
        string::utf8(b"iBank Canadian Dollar"),
        string::utf8(b"ICAD"),
        6,                                    // decimals
        string::utf8(b"CAD"),                 // peg: Canadian Dollar
        2,                                    // peg decimals
        0,                                    // max_supply: unlimited
        10000,                                // min_reserve_ratio: 100%
        3600,                                 // max_attestation_age: 1 hour
        10000000000000,                       // epoch_mint_limit: 10M
        10000000000000,                       // epoch_burn_limit: 10M
        1000000000000,                        // single_mint_limit: 1M
        1000000000000,                        // single_burn_limit: 1M
        string::utf8(b"https://creditchain.org/assets/icad-icon.png"),
        string::utf8(b"https://icad.creditchain.org"),
    );

    // Registry now has 6 entries: IUSD (#1), IEUR (#2), IJPY (#3), IGBP (#4), ICNY (#5), ICAD (#6)
    // All 6 are identical in structure — only config differs (IJPY uses 0 decimals)
    // Additional stablecoins can be created at runtime by any KYC_INSTITUTIONAL entity
}
```

### 5.2 Native Stablecoin Properties

| Property | IUSD | IEUR | IJPY | IGBP | ICNY | ICAD |
|----------|------|------|------|------|------|------|
| Registry entry | #1 | #2 | #3 | #4 | #5 | #6 |
| Peg currency | USD | EUR | JPY | GBP | CNY | CAD |
| ISO 4217 | 840 | 978 | 392 | 826 | 156 | 124 |
| Decimals | 6 | 6 | **0** | 6 | 6 | 6 |
| Max supply | Unlimited | Unlimited | Unlimited | Unlimited | Unlimited | Unlimited |
| Reserve ratio | ≥100% | ≥100% | ≥100% | ≥100% | ≥100% | ≥100% |
| Attestation freshness | 1 hour | 1 hour | 1 hour | 1 hour | 1 hour | 1 hour |
| Epoch mint limit | 10M | 10M | 1.5B | 10M | 10M | 10M |
| Single tx limit | 1M | 1M | 150M | 1M | 1M | 1M |
| Issuer at genesis | Framework | Framework | Framework | Framework | Framework | Framework |
| Bridge support | ETH, BSC | Ethereum | Ethereum | Ethereum | — (P2) | — (P2) |

> **IJPY note:** JPY is a zero-decimal currency per ISO 4217. Rate limits for IJPY are
> denominated in whole yen (1.5B JPY ≈ 10M USD at ~150 JPY/USD).

### 5.3 Cross-Currency Atomic Settlement (Native FX)

The 6 native stablecoins create an **on-chain forex market** with sub-second settlement:

```
IUSD ←→ IEUR    (USD/EUR)     Oracle-priced, atomic
IUSD ←→ IJPY    (USD/JPY)     Oracle-priced, atomic
IUSD ←→ IGBP    (USD/GBP)     Oracle-priced, atomic
IUSD ←→ ICNY    (USD/CNY)     Oracle-priced, atomic
IUSD ←→ ICAD    (USD/CAD)     Oracle-priced, atomic
IEUR ←→ IJPY    (EUR/JPY)     Oracle-priced, atomic
IEUR ←→ IGBP    (EUR/GBP)     Oracle-priced, atomic
IEUR ←→ ICNY    (EUR/CNY)     Oracle-priced, atomic
IEUR ←→ ICAD    (EUR/CAD)     Oracle-priced, atomic
IJPY ←→ IGBP    (JPY/GBP)     Oracle-priced, atomic
IJPY ←→ ICNY    (JPY/CNY)     Oracle-priced, atomic
IJPY ←→ ICAD    (JPY/CAD)     Oracle-priced, atomic
IGBP ←→ ICNY    (GBP/CNY)     Oracle-priced, atomic
IGBP ←→ ICAD    (GBP/CAD)     Oracle-priced, atomic
ICNY ←→ ICAD    (CNY/CAD)     Oracle-priced, atomic
```

**Total cross-pairs: C(6,2) = 15 native forex pairs at genesis.**

This replaces traditional FX settlement (T+2 via CLS Bank) with
**T+0 atomic settlement in < 1 second** on CreditChain.

### 5.4 Compatibility Layers

Each native stablecoin gets a thin compatibility module for direct access:

```move
module creditchain_framework::iusd_compat {
    fun iusd_addr(): address { stablecoin_registry::get_by_symbol(utf8(b"IUSD")) }
    public entry fun mint(issuer: &signer, to: address, amount: u64, purpose: String) { ... }
    public entry fun burn(issuer: &signer, from: address, amount: u64, purpose: String) { ... }
    #[view] public fun total_supply(): u64 { ... }
}

module creditchain_framework::ieur_compat {
    fun ieur_addr(): address { stablecoin_registry::get_by_symbol(utf8(b"IEUR")) }
    public entry fun mint(issuer: &signer, to: address, amount: u64, purpose: String) { ... }
    public entry fun burn(issuer: &signer, from: address, amount: u64, purpose: String) { ... }
    #[view] public fun total_supply(): u64 { ... }
}

module creditchain_framework::ijpy_compat { /* same pattern, 0 decimals */ }
module creditchain_framework::igbp_compat { /* same pattern */ }
module creditchain_framework::icny_compat { /* same pattern */ }
module creditchain_framework::icad_compat { /* same pattern */ }
```

### 5.5 Backward Compatibility

| Feature | Before (Hardcoded) | After (Factory Instance) | Breaking? |
|---------|--------------------|-----------------------------|-----------|
| Mint IUSD | `0x1::iusd::mint(...)` | `0x1::stablecoin_factory::mint(IUSD_ADDR, ...)` | API change |
| Burn IUSD | `0x1::iusd::burn(...)` | `0x1::stablecoin_factory::burn(IUSD_ADDR, ...)` | API change |
| Check supply | `0x1::iusd::total_supply()` | `0x1::stablecoin_factory::total_supply(IUSD_ADDR)` | API change |
| Hold IUSD | `CoinStore<IUSD>` at account | `PrimaryFungibleStore` at account | Transparent |
| Transfer IUSD | `coin::transfer<IUSD>(...)` | `primary_fungible_store::transfer(IUSD_META, ...)` | API change |
| Bridge IUSD | Per-coin bridge config | Auto via factory bridge adapter | Transparent |

**Compatibility layer:** A thin `0x1::iusd_compat` module wraps factory calls with
the old function signatures for smooth migration:

```move
module creditchain_framework::iusd_compat {
    const IUSD_METADATA_ADDR: address = @iusd_metadata;

    public entry fun mint(issuer: &signer, to: address, amount: u64, purpose: String) {
        stablecoin_factory::mint(issuer, IUSD_METADATA_ADDR, to, amount, purpose);
    }

    public entry fun burn(issuer: &signer, from: address, amount: u64, purpose: String) {
        stablecoin_factory::burn(issuer, IUSD_METADATA_ADDR, from, amount, purpose);
    }

    #[view]
    public fun total_supply(): u64 {
        stablecoin_factory::total_supply(IUSD_METADATA_ADDR)
    }
}
```

---

## 6. OpenIBank Integration

### 6.1 Off-Chain Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    OpenIBank Platform                           │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Stablecoin Management Service (new crate)               │   │
│  │  ┌───────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │ Factory API   │  │ Issuer Mgmt  │  │ Reserve Mgmt │   │   │
│  │  │ create/list   │  │ per-coin     │  │ per-coin     │   │   │
│  │  │ config/pause  │  │ WorldLine    │  │ attestation  │   │   │
│  │  └──────┬────────┘  └──────┬───────┘  └──────┬───────┘   │   │
│  │         │                 │                  │           │   │
│  │  ┌──────┴─────────────────┴──────────────────┴────────┐  │   │
│  │  │     CreditChain Client (REST + Transaction)        │  │   │
│  │  │     ├─ Submit create_stablecoin() txn              │  │   │
│  │  │     ├─ Submit mint() / burn() txns                 │  │   │
│  │  │     ├─ Submit attest_reserves() txns               │  │   │
│  │  │     ├─ Query registry for discovery                │  │   │
│  │  │     └─ Poll finality confirmations                 │  │   │
│  │  └────────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                 │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Existing OpenIBank Services                             │   │
│  │  ├─ WorldLineIssuer (generalized to multi-coin)          │   │
│  │  ├─ Ledger (double-entry, per-stablecoin asset accounts) │   │
│  │  ├─ Clearing (cross-stablecoin netting)                  │   │
│  │  ├─ Settlement (DvP with any stablecoin)                 │   │
│  │  └─ Policy Engine (per-stablecoin risk limits)           │   │
│  └──────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Generalized Issuer Pattern

The existing `WorldLineIssuer` generalizes from single-coin to multi-coin:

```rust
// Before: Single IUSD issuer
pub struct WorldLineIssuer {
    inner: Issuer,
    wll_log: Arc<RwLock<Vec<IssuerWllEvent>>>,
}

// After: Multi-coin issuer factory
pub struct StablecoinIssuerFactory {
    /// Per-stablecoin issuer instances
    issuers: HashMap<StablecoinId, StablecoinIssuer>,
    /// CreditChain client for on-chain operations
    cc_client: Arc<CreditChainClient>,
}

pub struct StablecoinIssuer {
    /// Stablecoin identity
    id: StablecoinId,
    symbol: String,
    metadata_addr: Address,
    /// Reserve tracking
    reserve: Arc<RwLock<ReserveModel>>,
    /// Issuance policy
    policy: Arc<RwLock<IssuancePolicy>>,
    /// State
    state: Arc<RwLock<IssuerState>>,
    /// WorldLine event log (per-stablecoin)
    wll_log: Arc<RwLock<Vec<IssuerWllEvent>>>,
    /// Signing keypair (for CreditChain transactions)
    keypair: Keypair,
    /// Off-chain ledger integration
    ledger: Arc<Ledger>,
}

impl StablecoinIssuerFactory {
    /// Create a new stablecoin (submits tx to CreditChain)
    pub async fn create_stablecoin(&self, config: CreateStablecoinRequest)
        -> Result<StablecoinIssuer> { /* ... */ }

    /// Get issuer for existing stablecoin
    pub fn get_issuer(&self, symbol: &str) -> Option<&StablecoinIssuer> { /* ... */ }

    /// List all managed stablecoins
    pub fn list_stablecoins(&self) -> Vec<StablecoinSummary> { /* ... */ }
}
```

### 6.3 REST API

```yaml
# OpenAPI 3.1 — Stablecoin Management API
paths:

  /v1/stablecoin/create:
    post:
      summary: Create a new stablecoin (one-click)
      description: |
        Creates a fully operational stablecoin on CreditChain in a single
        API call. Requires KYC_INSTITUTIONAL (level 4) verification.
        Returns immediately with stablecoin metadata; on-chain creation
        confirms within ~1 second.
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required: [name, symbol, peg_currency]
              properties:
                name:
                  type: string
                  example: "Acme Stable Dollar"
                symbol:
                  type: string
                  example: "ACSD"
                  pattern: "^[A-Z]{2,10}$"
                decimals:
                  type: integer
                  default: 6
                  minimum: 0
                  maximum: 18
                peg_currency:
                  type: string
                  enum: [USD, EUR, GBP, SGD, JPY, CHF, AUD, CAD, HKD, XAU, XAG, XDR]
                  example: "USD"
                max_supply:
                  type: integer
                  default: 0
                  description: "0 = unlimited"
                reserve_config:
                  type: object
                  properties:
                    min_ratio_percent:
                      type: number
                      default: 100
                    max_attestation_age_minutes:
                      type: integer
                      default: 60
                rate_limits:
                  type: object
                  properties:
                    epoch_mint_limit:
                      type: integer
                      default: 1000000000000
                    single_mint_limit:
                      type: integer
                      default: 100000000000
                icon_uri:
                  type: string
                project_uri:
                  type: string
      responses:
        '201':
          description: Stablecoin created
          content:
            application/json:
              schema:
                type: object
                properties:
                  metadata_address:
                    type: string
                    description: "On-chain object address"
                  symbol:
                    type: string
                  name:
                    type: string
                  tx_hash:
                    type: string
                    description: "CreditChain transaction hash"
                  status:
                    type: string
                    enum: [confirmed, pending]
                  dashboard_url:
                    type: string
                    description: "Management dashboard URL"

  /v1/stablecoin/list:
    get:
      summary: List all stablecoins on CreditChain
      parameters:
        - name: peg_currency
          in: query
          schema: { type: string }
        - name: offset
          in: query
          schema: { type: integer, default: 0 }
        - name: limit
          in: query
          schema: { type: integer, default: 50 }

  /v1/stablecoin/{symbol}:
    get:
      summary: Get stablecoin details
      parameters:
        - name: symbol
          in: path
          required: true
          schema: { type: string }
      responses:
        '200':
          content:
            application/json:
              schema:
                type: object
                properties:
                  metadata_address: { type: string }
                  name: { type: string }
                  symbol: { type: string }
                  decimals: { type: integer }
                  peg_currency: { type: string }
                  total_supply: { type: integer }
                  reserve_ratio_percent: { type: number }
                  reserve_attestation_age_secs: { type: integer }
                  is_paused: { type: boolean }
                  issuer_count: { type: integer }
                  created_at: { type: string, format: date-time }
                  creator: { type: string }

  /v1/stablecoin/{symbol}/mint:
    post:
      summary: Mint tokens
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required: [recipient, amount]
              properties:
                recipient: { type: string }
                amount: { type: integer }
                purpose: { type: string }

  /v1/stablecoin/{symbol}/burn:
    post:
      summary: Burn tokens
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required: [from, amount]
              properties:
                from: { type: string }
                amount: { type: integer }
                purpose: { type: string }

  /v1/stablecoin/{symbol}/attest:
    post:
      summary: Submit reserve attestation
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required: [total_reserves_value, proof_hash]
              properties:
                total_reserves_value: { type: integer }
                proof_hash: { type: string, format: hex }

  /v1/stablecoin/{symbol}/supply:
    get:
      summary: Get supply and reserve details

  /v1/stablecoin/{symbol}/pause:
    post:
      summary: Emergency pause

  /v1/stablecoin/{symbol}/config:
    put:
      summary: Update stablecoin configuration

  /v1/stablecoin/swap:
    post:
      summary: Atomic cross-stablecoin swap
      requestBody:
        content:
          application/json:
            schema:
              type: object
              required: [from_symbol, to_symbol, from_amount]
              properties:
                from_symbol: { type: string, example: "IUSD" }
                to_symbol: { type: string, example: "EUSD" }
                from_amount: { type: integer }
                min_to_amount: { type: integer }
```

### 6.4 SDK Integration

```typescript
// TypeScript SDK — @creditchain/stablecoin-sdk
import { CreditChainClient, StablecoinFactory } from '@creditchain/sdk';

const client = new CreditChainClient('https://api.creditchain.org/v1');
const factory = new StablecoinFactory(client);

// ONE-CLICK: Create a stablecoin
const coin = await factory.create({
    name: "Acme Stable Dollar",
    symbol: "ACSD",
    decimals: 6,
    pegCurrency: "USD",
    reserveConfig: {
        minRatioPercent: 100,
        maxAttestationAgeMinutes: 60,
    },
});
console.log(`Created ${coin.symbol} at ${coin.metadataAddress}`);

// Mint tokens
await coin.mint({
    recipient: "0xRECIPIENT",
    amount: 1_000_000,  // 1 ACSD
    purpose: "Customer deposit",
});

// Attest reserves
await coin.attestReserves({
    totalReservesValue: 50_000_000_000,  // $50M
    proofHash: "0x...",
});

// Check supply
const supply = await coin.totalSupply();
console.log(`Total supply: ${supply.formatted}`);

// Cross-stablecoin swap
await factory.swap({
    fromSymbol: "IUSD",
    toSymbol: "EUSD",
    fromAmount: 1_000_000,
    minToAmount: 920_000,  // Slippage protection (EUR/USD rate)
});

// List all stablecoins
const all = await factory.list({ pegCurrency: "USD" });
console.log(`${all.length} USD-pegged stablecoins on CreditChain`);
```

```python
# Python SDK — creditchain-sdk
from creditchain import CreditChainClient, StablecoinFactory

client = CreditChainClient("https://api.creditchain.org/v1")
factory = StablecoinFactory(client)

# ONE-CLICK: Create stablecoin
coin = factory.create(
    name="Acme Stable Dollar",
    symbol="ACSD",
    decimals=6,
    peg_currency="USD",
)

# Mint
coin.mint(recipient="0xRECIPIENT", amount=1_000_000, purpose="Deposit")
```

---

## 7. Deployment Model Integration

### 7.1 Model Matrix

| Model | Who Creates | KYC Requirement | Governance | Registry |
|-------|-------------|-----------------|------------|----------|
| **Public Mainnet** | Any institution | KYC_INSTITUTIONAL (L4) | On-chain vote for upgrades | Global shared |
| **Consortium** | Member institutions | Consortium agreement | Multi-party vote | Consortium-scoped |
| **Private Enterprise** | Single organization | Internal policy | Admin-only | Org-private |
| **Sovereign CBDC** | Central bank | Government authority | Regulatory | National |

### 7.2 Sovereign CBDC Use Case

```
Central Bank deploys private CreditChain instance
    │
    ├── create_stablecoin("Digital Dollar", "DCNY", 2, "CNY", ...)
    │   ├── Central bank is sole issuer
    │   ├── Reserve = government backing (100% guaranteed)
    │   ├── No attestation needed (sovereign guarantee)
    │   └── Rate limits = national monetary policy
    │
    ├── create_stablecoin("Wholesale CBDC", "wDCNY", 6, "CNY", ...)
    │   ├── Interbank settlement only
    │   ├── High rate limits
    │   └── Restricted to licensed banks
    │
    └── Both trade on same CreditChain instance
        ├── Atomic swaps between retail & wholesale
        ├── Settlement via DvP
        └── WorldLine audit trail for regulators
```

### 7.3 White-Label Integration (Enterprise)

```
Enterprise Customer (e.g., ACME Bank)
    │
    ├── Signs up on OpenIBank Enterprise tier
    ├── Completes KYB verification (L3-Inst)
    ├── Receives API credentials
    │
    └── POST /v1/stablecoin/create
        {
            "name": "ACME Dollar",
            "symbol": "ACMD",
            "peg_currency": "USD",
            "icon_uri": "https://acme.bank/acmd-icon.png"
        }
        │
        └── Response (< 2 seconds):
            {
                "metadata_address": "0x...",
                "symbol": "ACMD",
                "status": "confirmed",
                "dashboard_url": "https://app.openibank.io/stablecoin/ACMD"
            }
            │
            └── ACME Bank now has:
                ├── Fully branded stablecoin (ACMD)
                ├── Management dashboard
                ├── API access for mint/burn/attest
                ├── Bridge to Ethereum (wACMD ERC-20)
                ├── Interoperable with IUSD, EUSD, etc.
                └── Full WorldLine audit trail
```

---

## 8. Fee Model

### 8.1 Fee Schedule

| Operation | Fee | Denominated In | Recipient |
|-----------|-----|----------------|-----------|
| Create stablecoin | 10,000 CCC | CCC (native gas) | Platform treasury |
| Mint tokens | 5 bps (0.05%) | Minted stablecoin | Platform fee pool |
| Burn tokens | 5 bps (0.05%) | Burned stablecoin | Platform fee pool |
| Reserve attestation | Gas only | CCC | Validators |
| Cross-stablecoin swap | 10 bps (0.10%) | From stablecoin | Platform fee pool |
| Bridge out | 10 bps (0.10%) | Bridged stablecoin | Bridge operators |
| Admin operations | Gas only | CCC | Validators |

### 8.2 Fee Distribution (Platform Fee Pool)

```
Platform Fees Collected
    ├── 40% → CCC Burn (deflationary)
    ├── 25% → Treasury (development)
    ├── 20% → Stakers (validator rewards)
    ├── 10% → Clearing Fund (risk buffer)
    └──  5% → Referrers (growth incentive)
```

### 8.3 Institution Revenue Share

Stablecoin creators can configure their own fees ON TOP of platform fees:

```move
// Issuer-level fee configuration
struct IssuerFeeConfig {
    issuer_mint_fee_bps: u64,   // Issuer's additional fee on minting
    issuer_burn_fee_bps: u64,   // Issuer's additional fee on burning
    fee_recipient: address,      // Where issuer fees go
}

// Total fee to user = platform_fee + issuer_fee
// Example: Platform 5 bps + Issuer 10 bps = 15 bps total on mint
```

---

## 9. Security Model

### 9.1 Access Control Layers

```
Layer 1: KYC Gate
    └── create_stablecoin() requires KYC_INSTITUTIONAL (L4)

Layer 2: Role-Based Access
    ├── Admin: config, pause, freeze, issuer/auditor management
    ├── Issuer: mint, burn
    └── Auditor: attest_reserves

Layer 3: Rate Limiting
    ├── Per-epoch volume caps (mint + burn)
    └── Per-transaction caps

Layer 4: Reserve Enforcement
    ├── Mint blocked if attestation stale
    └── Mint blocked if reserve ratio would drop below minimum

Layer 5: Circuit Breaker
    └── Admin can pause ALL operations instantly

Layer 6: Account Freeze
    └── Admin can freeze individual accounts (compliance)
```

### 9.2 Threat Mitigation

| Threat | Mitigation |
|--------|------------|
| Unauthorized minting | Issuer role check + reserve ratio enforcement |
| Reserve attestation fraud | Multiple auditors + on-chain proof hash + WorldLine anchor |
| Infinite supply attack | max_supply cap + epoch rate limits |
| Admin key compromise | Multi-sig admin (via vault module) + time-lock |
| Cross-stablecoin manipulation | Oracle price feeds from multiple sources |
| Compliance violation | KYC gate + account freeze + WorldLine audit trail |
| Symbol squatting | Creation fee (10,000 CCC) + KYC requirement |

---

## 10. WorldLine Integration

Every stablecoin operation automatically creates a WorldLine anchor:

| Operation | Anchor Type | Data Hash Contains |
|-----------|-------------|-------------------|
| Create stablecoin | ANCHOR_COMPLIANCE | name, symbol, creator, config |
| Mint | ANCHOR_RESERVE | metadata_addr, recipient, amount, supply_after |
| Burn | ANCHOR_RESERVE | metadata_addr, from, amount, supply_after |
| Attest reserves | ANCHOR_AUDIT | metadata_addr, reserves, ratio, proof_hash |
| Pause/Unpause | ANCHOR_COMPLIANCE | metadata_addr, admin, paused_state |
| Freeze account | ANCHOR_COMPLIANCE | metadata_addr, admin, frozen_account |

---

## 11. Invariants

| ID | Invariant | Enforcement |
|----|-----------|-------------|
| OCS-1 | Symbol MUST be unique across all stablecoins | Registry lookup before creation |
| OCS-2 | Minting REQUIRES fresh reserve attestation (< max_age) | Timestamp check in mint() |
| OCS-3 | Minting REQUIRES reserve ratio >= min_ratio after mint | Ratio calculation in mint() |
| OCS-4 | Total supply of each stablecoin MUST equal sum of all balances | FA V2 concurrent supply tracking |
| OCS-5 | Only authorized issuers can mint/burn | Signer + governance check |
| OCS-6 | Only authorized auditors can attest | Signer + governance check |
| OCS-7 | Only admin can pause/freeze/configure | Signer + governance check |
| OCS-8 | Pause MUST block ALL operations except unpause | First check in every entry function |
| OCS-9 | Frozen account MUST NOT receive or send tokens | Check in mint/burn/transfer |
| OCS-10 | Cross-stablecoin swap MUST be atomic | Single Move transaction |
| OCS-11 | Platform fees MUST be collected before delivery | Fee deduction in mint/burn |
| OCS-12 | Every operation MUST emit event + WorldLine anchor | Event + worldline::anchor() in every fn |
| OCS-13 | Creation fee MUST be paid in CCC | CCC transfer in create_stablecoin() |
| OCS-14 | Native Big 6 (IUSD, IEUR, IJPY, IGBP, ICNY, ICAD) MUST be entries #1-#6 in registry | Genesis initialization order |
| OCS-15 | Rate limits MUST reset at epoch boundary | Epoch check in rate tracker |
| OCS-16 | Native Big 6 symbols MUST NOT be deletable or transferable | Framework-owned, no admin transfer |
| OCS-17 | Cross-currency swaps MUST use oracle price < 60 seconds old | Oracle freshness check in swap |
| OCS-18 | All 15 native cross-pairs (C(6,2)) MUST be swappable at genesis | Oracle feeds for USD/EUR/JPY/GBP/CNY/CAD |
| OCS-19 | IJPY MUST use 0 decimals (JPY has no minor units per ISO 4217) | Enforced at genesis creation |

---

## 12. Implementation Roadmap

| Phase | Scope | Timeline |
|-------|-------|----------|
| P9a | StablecoinFactory Move module (create, mint, burn, attest, pause) | Week 1-2 |
| P9b | StablecoinRegistry Move module (register, lookup, list) | Week 2 |
| P9c | IUSD migration to factory instance (genesis + compat layer) | Week 3 |
| P9d | Cross-stablecoin swap module | Week 3-4 |
| P9e | OpenIBank REST API + Rust service crate | Week 4-5 |
| P9f | TypeScript + Python SDK | Week 5-6 |
| P9g | Management dashboard (web UI) | Week 6-8 |
| P9h | Bridge adapter (any stablecoin → Ethereum ERC-20) | Week 8-10 |
| P9i | Load testing + security audit | Week 10-12 |

---

*CreditChain One-Click Stablecoin — Every Institution, Every Currency, One Transaction*
