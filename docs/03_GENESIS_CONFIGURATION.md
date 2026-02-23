# CreditChain Genesis Configuration

> Document 03 | CreditChain Design Series | Version 1.0
> Scope: Chain genesis, CCC token, initial validator set, epoch configuration

---

## 1. Genesis Overview

The CreditChain genesis defines the initial state of the blockchain — the first block
from which all subsequent state derives. It includes the framework modules, initial
accounts, CCC token configuration, and validator set.

### Genesis Parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| Chain ID | 0xCC01 (52225) | Unique to CreditChain mainnet |
| Chain ID (Testnet) | 0xCC02 (52226) | CreditChain testnet |
| Chain ID (Devnet) | 0xCC03 (52227) | CreditChain devnet |
| Genesis timestamp | Deploy time | Unix epoch seconds |
| Epoch duration | 7200 seconds (2 hours) | Validator rotation |
| Min stake | 1,000,000 CCC | Validator minimum |
| Max stake | 50,000,000 CCC | Concentration limit |
| Voting power cap | 10% | No validator > 10% total |
| Rewards rate | 7% annual (Year 1) | Declining schedule |

---

## 2. CCC Token Genesis

### 2.1 Total Supply

```
Total Supply:    1,000,000,000 CCC (1 Billion)
Decimals:        8
Smallest Unit:   1 Octa = 0.00000001 CCC
Total Octas:     100,000,000,000,000,000 (10^17)
```

### 2.2 Initial Distribution

| Account | Purpose | CCC Amount | Percentage |
|---------|---------|-----------|-----------|
| 0xCC01...0001 | Ecosystem Fund | 300,000,000 | 30% |
| 0xCC01...0002 | Development Fund | 250,000,000 | 25% |
| 0xCC01...0003 | Foundation Reserve | 200,000,000 | 20% |
| 0xCC01...0004 | Staking Rewards Pool | 150,000,000 | 15% |
| 0xCC01...0005 | Exchange Liquidity | 50,000,000 | 5% |
| 0xCC01...0006 | Advisors & Partners | 50,000,000 | 5% |

### 2.3 Vesting Schedule

```
Ecosystem (30%):     4-year linear, 0 cliff
                     ├── 6.25M/month unlocked
                     └── Used for grants, airdrops, incentives

Development (25%):   4-year linear, 1-year cliff
                     ├── Year 1: 0 (locked)
                     └── Year 2-4: ~6.94M/month

Foundation (20%):    Governance-locked
                     ├── Requires ⅔ validator vote to unlock
                     └── Emergency reserve for protocol

Staking (15%):       Released per epoch as rewards
                     ├── Year 1: 7% APR
                     ├── Year 2: 6% APR
                     ├── Year 3: 5% APR
                     └── Year 4+: 4% APR (floor)

Exchange (5%):       Immediate
                     └── Market making and liquidity

Advisors (5%):       3-year linear, 6-month cliff
                     └── ~1.39M/month after cliff
```

---

## 3. Genesis Validator Set

### 3.1 Phase 0 — Devnet (4 Validators)

All operated by iBank/OpenIBank:

| Validator | Location | Stake | Role |
|-----------|----------|-------|------|
| cc-validator-0 | US-East (AWS) | 10,000,000 CCC | Seed validator |
| cc-validator-1 | EU-West (AWS) | 10,000,000 CCC | Geographic diversity |
| cc-validator-2 | AP-Southeast (AWS) | 10,000,000 CCC | Asia coverage |
| cc-validator-3 | US-West (GCP) | 10,000,000 CCC | Cloud diversity |

### 3.2 Phase 1 — Testnet (7-21 Validators)

```
Genesis Validators (4):     iBank-operated (from devnet)
Invited Validators (3-17):  Partner institutions
                            - Exchange partners
                            - Financial institutions
                            - Infrastructure providers
```

### 3.3 Validator Key Structure

Each validator requires 4 key pairs:

| Key | Algorithm | Purpose |
|-----|-----------|---------|
| Account Key | Ed25519 | Account ownership |
| Consensus Key | Ed25519 | Block signing |
| Network Key | x25519 | P2P encryption (Noise) |
| Validator Network Key | x25519 | Validator-only mesh |

### 3.4 Genesis Key Ceremony

```
Step 1: Each validator generates keys locally
        creditchain keygen --output validator-keys.yaml

Step 2: Public keys submitted to genesis coordinator
        creditchain genesis submit-keys \
            --validator-config validator-keys.yaml \
            --genesis-coordinator https://genesis.creditchain.io

Step 3: Genesis coordinator aggregates and generates genesis blob
        creditchain genesis generate \
            --chain-id 0xCC01 \
            --framework creditchain-framework \
            --validators validators.yaml \
            --output genesis.blob

Step 4: All validators verify genesis blob hash
        creditchain genesis verify --genesis genesis.blob

Step 5: Validators start nodes with verified genesis
        creditchain-node --config creditchain-node.yaml
```

---

## 4. Framework Modules at Genesis

### 4.1 Core Modules (Deployed at 0x1)

```move
// Standard library (from move-stdlib)
0x1::vector
0x1::string
0x1::option
0x1::error
0x1::signer
0x1::hash
0x1::bcs

// CreditChain framework
0x1::creditchain_account     // Account creation & management
0x1::creditchain_coin        // CCC token (native coin)
0x1::coin                    // Generic coin standard
0x1::creditchain_governance  // On-chain governance
0x1::staking                 // Validator staking
0x1::staking_config          // Staking parameters
0x1::block                   // Block metadata
0x1::timestamp               // Time oracle
0x1::chain_id                // Chain identification
0x1::reconfiguration         // Epoch reconfiguration
0x1::transaction_fee          // Fee collection
0x1::gas_schedule            // Gas pricing
0x1::version                 // Protocol version
0x1::chain_status            // Genesis / operating status

// CreditChain custom modules (NEW)
0x1::iusd                    // IUSD stablecoin (empty scaffold at genesis)
0x1::settlement              // DvP settlement (empty scaffold)
0x1::bridge                  // Cross-chain bridge (empty scaffold)
0x1::worldline               // Event anchoring (empty scaffold)
0x1::credit_score            // Credit scoring (empty scaffold)
0x1::compliance              // KYC attestations (empty scaffold)
```

### 4.2 Token Standard (Deployed at 0x3)

```move
0x3::creditchain_token       // NFT/token standard
0x3::token                   // Token operations
0x3::token_transfers         // Transfer logic
```

---

## 5. Genesis Configuration File

### 5.1 genesis.yaml

```yaml
# CreditChain Genesis Configuration
chain_id: 52225  # 0xCC01
epoch_duration_secs: 7200
min_stake: 100000000000000  # 1,000,000 CCC in Octas
max_stake: 5000000000000000  # 50,000,000 CCC in Octas
voting_power_increase_limit: 10
rewards_apy_percentage: 7
min_price_per_gas_unit: 100
max_gas_per_transaction: 2000000

# CCC Token
total_supply: 100000000000000000  # 1B CCC in Octas
token_name: "CreditChain Coin"
token_symbol: "CCC"
token_decimals: 8

# Initial accounts
accounts:
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000001"
    balance: 30000000000000000  # 300M CCC = Ecosystem Fund
    role: "ecosystem_fund"
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000002"
    balance: 25000000000000000  # 250M CCC = Development
    role: "development_fund"
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000003"
    balance: 20000000000000000  # 200M CCC = Foundation
    role: "foundation_reserve"
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000004"
    balance: 15000000000000000  # 150M CCC = Staking Rewards
    role: "staking_pool"
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000005"
    balance: 5000000000000000   # 50M CCC = Exchange Liquidity
    role: "exchange_liquidity"
  - address: "0xCC0100000000000000000000000000000000000000000000000000000000006"
    balance: 5000000000000000   # 50M CCC = Advisors
    role: "advisors"

# Validators
validators:
  - name: "cc-validator-0"
    host: "validator-0.creditchain.io"
    port: 6182
    stake: 1000000000000000  # 10M CCC
    consensus_public_key: "<generated>"
    network_public_key: "<generated>"
    account_address: "<generated>"

# Framework
framework_path: "creditchain-move/framework"
move_stdlib_path: "creditchain-move/framework/move-stdlib"
```

---

## 6. Epoch & Reconfiguration

### 6.1 Epoch Structure

```
Epoch N
├── Blocks [0 ... ~36000]       (2 hours @ 200ms block time)
├── Validator Set: frozen for entire epoch
├── Gas Schedule: frozen for entire epoch
├── Staking Rewards: calculated at epoch end
└── Reconfiguration Event at boundary

Epoch N+1
├── New validator set (if staking changes)
├── New gas schedule (if governance voted)
├── Rewards distributed from epoch N
└── Continue...
```

### 6.2 Reconfiguration Parameters

| Parameter | Value | Governance-Changeable |
|-----------|-------|----------------------|
| epoch_duration_secs | 7200 | Yes (min 3600, max 86400) |
| min_stake | 1,000,000 CCC | Yes |
| max_stake | 50,000,000 CCC | Yes |
| rewards_apy | 7% (Year 1) | Yes (min 1%, max 20%) |
| min_gas_price | 100 Octa | Yes |
| max_gas_per_txn | 2,000,000 | Yes |
| voting_power_cap | 10% | Yes (min 5%, max 25%) |

---

## 7. Genesis Build Process

### 7.1 Build Steps

```bash
# 1. Compile Move framework
cd creditchain-move/framework
creditchain move compile --named-addresses creditchain_framework=0x1

# 2. Generate genesis blob
creditchain genesis generate \
    --config genesis.yaml \
    --framework-path creditchain-move/framework/build \
    --output genesis.blob \
    --waypoint-output waypoint.txt

# 3. Verify genesis
creditchain genesis verify \
    --genesis genesis.blob \
    --expected-chain-id 52225

# 4. Distribute genesis blob to validators
# Each validator receives: genesis.blob + waypoint.txt

# 5. Start first validator
creditchain-node \
    --config creditchain-node.yaml \
    --genesis genesis.blob \
    --waypoint "$(cat waypoint.txt)"
```

### 7.2 Genesis Verification

After genesis, verify:
- Chain ID matches 0xCC01
- Total CCC supply equals 1 billion
- All framework modules deployed at 0x1
- All genesis accounts have correct balances
- Validator set matches expected configuration
- First epoch starts correctly

---

## 8. Post-Genesis Operations

### 8.1 First Transactions

After genesis boot:

1. **Verify framework**: Query all modules at 0x1
2. **Test transfer**: Send CCC between genesis accounts
3. **Deploy IUSD scaffold**: Initialize IUSD module with issuer authority
4. **Configure bridge**: Set up bridge authority accounts
5. **Initialize governance**: Set up governance module parameters

### 8.2 Faucet (Devnet/Testnet Only)

```bash
# Start faucet service
creditchain-faucet-service \
    --chain-id 52226 \  # testnet
    --mint-key mint-key.yaml \
    --max-amount 10000000000000  # 100K CCC per request
    --port 8081

# Request test tokens
curl -X POST https://faucet.creditchain.io/mint \
    -H "Content-Type: application/json" \
    -d '{"address":"0xYOUR_ADDRESS","amount":1000000000000}'
```

---

## 9. Multi-Network Genesis

| Network | Chain ID | Genesis Hash | Purpose |
|---------|----------|-------------|---------|
| Mainnet | 0xCC01 | TBD | Production |
| Testnet | 0xCC02 | TBD | Public testing |
| Devnet | 0xCC03 | TBD | Development |
| Local | 0xCC04 | Generated | Local testing |

---

## 10. Invariants

| ID | Invariant |
|----|-----------|
| GEN-1 | CCC total supply at genesis MUST equal exactly 1,000,000,000 CCC (10^17 Octas) |
| GEN-2 | Sum of all genesis account balances MUST equal total supply |
| GEN-3 | Chain ID MUST be unique across all networks (0xCC01, 0xCC02, 0xCC03) |
| GEN-4 | Minimum 4 validators required for genesis (BFT: 3f+1, f=1) |
| GEN-5 | All validators MUST verify genesis blob hash before starting |
| GEN-6 | Epoch duration MUST be ≥ 3600 seconds (1 hour minimum) |
| GEN-7 | No single validator stake MAY exceed voting power cap at genesis |

---

*CreditChain Genesis — The First Block of AI-Era Finance*
