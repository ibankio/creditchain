# CreditChain Rebranding Guide: Libra2 → CreditChain

> Document 02 | CreditChain Design Series | Version 1.0
> Scope: Complete renaming strategy for 200+ crates, configs, docs, and binaries

---

## 1. Rebranding Scope

### 1.1 Naming Convention

| Old (Libra2) | New (CreditChain) | Short Form |
|--------------|-------------------|------------|
| libra2 | creditchain | cc |
| Libra2 | CreditChain | CC |
| LIBRA2 | CREDITCHAIN | CC |
| libra2-core | creditchain-core | — |
| libra2-node | creditchain-node | cc-node |
| libra2-framework | creditchain-framework | cc-framework |
| L2 (coin ticker) | CCC | CCC |

### 1.2 Domain & URLs

| Item | Old | New |
|------|-----|-----|
| GitHub | libra2org/libra2-core | ibankio/creditchain |
| Website | libra2.org | creditchain.io |
| Docs | docs.libra2.org | docs.creditchain.io |
| Explorer | explorer.libra2.org | explorer.creditchain.io |
| Discord | discord.gg/libra2 | discord.gg/creditchain |
| Docker Hub | libra2/libra2-node | ibankio/creditchain-node |

---

## 2. Phased Rebranding Strategy

### Phase 0: Non-Breaking Changes (Week 1)
- Update README.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md
- Update LICENSE headers
- Update Docker image names
- Update CI/CD references
- Add CreditChain banner/logo assets

### Phase 1: Config & User-Facing (Week 2)
- Rename config files: `libra2-node.yaml` → `creditchain-node.yaml`
- Update CLI binary name: `libra2` → `creditchain`
- Update environment variable prefixes: `LIBRA2_*` → `CREDITCHAIN_*`
- Update log prefixes and metrics namespaces
- Update API response headers and metadata

### Phase 2: Crate Names (Week 3-4)
- Rename all Cargo.toml `[package] name` fields
- Update all `use libra2_*` imports
- Update all inter-crate dependency references
- This is the largest phase — 200+ crates

### Phase 3: Move Framework (Week 5)
- Rename Move module references
- Update named addresses in Move.toml files
- Update framework cached packages
- Rebuild genesis blob with new module names

### Phase 4: Cleanup & Verification (Week 6)
- Full `cargo build` verification
- Full test suite pass
- Docker build verification
- Documentation audit
- Remove all remaining Libra2 references

---

## 3. Crate Renaming Master List

### 3.1 Core Crates (crates/ directory)

| # | Old Crate Name | New Crate Name | Path |
|---|---------------|----------------|------|
| 1 | libra2 | creditchain | crates/libra2 → crates/creditchain |
| 2 | libra2-admin-service | creditchain-admin-service | crates/ |
| 3 | libra2-api-tester | creditchain-api-tester | crates/ |
| 4 | libra2-bcs-utils | creditchain-bcs-utils | crates/ |
| 5 | libra2-bitvec | creditchain-bitvec | crates/ |
| 6 | libra2-build-info | creditchain-build-info | crates/ |
| 7 | libra2-collections | creditchain-collections | crates/ |
| 8 | libra2-compression | creditchain-compression | crates/ |
| 9 | libra2-crypto | creditchain-crypto | crates/ |
| 10 | libra2-crypto-derive | creditchain-crypto-derive | crates/ |
| 11 | libra2-debugger | creditchain-debugger | crates/ |
| 12 | libra2-dkg | creditchain-dkg | crates/ |
| 13 | libra2-drop-helper | creditchain-drop-helper | crates/ |
| 14 | libra2-enum-conversion-derive | creditchain-enum-conversion-derive | crates/ |
| 15 | libra2-faucet-cli | creditchain-faucet-cli | crates/ |
| 16 | libra2-faucet-core | creditchain-faucet-core | crates/ |
| 17 | libra2-faucet-metrics-server | creditchain-faucet-metrics-server | crates/ |
| 18 | libra2-faucet-service | creditchain-faucet-service | crates/ |
| 19 | libra2-genesis | creditchain-genesis | crates/ |
| 20 | libra2-github-client | creditchain-github-client | crates/ |
| 21 | libra2-id-generator | creditchain-id-generator | crates/ |
| 22 | libra2-infallible | creditchain-infallible | crates/ |
| 23 | libra2-inspection-service | creditchain-inspection-service | crates/ |
| 24 | libra2-jwk-consensus | creditchain-jwk-consensus | crates/ |
| 25 | libra2-keygen | creditchain-keygen | crates/ |
| 26 | libra2-ledger | creditchain-ledger | crates/ |
| 27 | libra2-localnet | creditchain-localnet | crates/ |
| 28 | libra2-log-derive | creditchain-log-derive | crates/ |
| 29 | libra2-logger | creditchain-logger | crates/ |
| 30 | libra2-metrics-core | creditchain-metrics-core | crates/ |
| 31 | libra2-network-checker | creditchain-network-checker | crates/ |
| 32 | libra2-node-identity | creditchain-node-identity | crates/ |
| 33 | libra2-openapi | creditchain-openapi | crates/ |
| 34 | libra2-profiler | creditchain-profiler | crates/ |
| 35 | libra2-proptest-helpers | creditchain-proptest-helpers | crates/ |
| 36 | libra2-push-metrics | creditchain-push-metrics | crates/ |
| 37 | libra2-rate-limiter | creditchain-rate-limiter | crates/ |
| 38 | libra2-rest-client | creditchain-rest-client | crates/ |
| 39 | libra2-retrier | creditchain-retrier | crates/ |
| 40 | libra2-rosetta | creditchain-rosetta | crates/ |
| 41 | libra2-rosetta-cli | creditchain-rosetta-cli | crates/ |
| 42 | libra2-runtimes | creditchain-runtimes | crates/ |
| 43 | libra2-speculative-state-helper | creditchain-speculative-state-helper | crates/ |
| 44 | libra2-system-utils | creditchain-system-utils | crates/ |
| 45 | libra2-telemetry | creditchain-telemetry | crates/ |
| 46 | libra2-telemetry-service | creditchain-telemetry-service | crates/ |
| 47 | libra2-temppath | creditchain-temppath | crates/ |
| 48 | libra2-time-service | creditchain-time-service | crates/ |
| 49 | libra2-transaction-filters | creditchain-transaction-filters | crates/ |
| 50 | libra2-warp-webserver | creditchain-warp-webserver | crates/ |

### 3.2 Move Crates (libra2-move/ directory)

| # | Old Crate Name | New Crate Name |
|---|---------------|----------------|
| 51 | libra2-abstract-gas-usage | creditchain-abstract-gas-usage |
| 52 | libra2-aggregator | creditchain-aggregator |
| 53 | libra2-gas-algebra | creditchain-gas-algebra |
| 54 | libra2-gas-calibration | creditchain-gas-calibration |
| 55 | libra2-gas-meter | creditchain-gas-meter |
| 56 | libra2-gas-profiling | creditchain-gas-profiling |
| 57 | libra2-gas-schedule | creditchain-gas-schedule |
| 58 | libra2-gas-schedule-updator | creditchain-gas-schedule-updator |
| 59 | libra2-memory-usage-tracker | creditchain-memory-usage-tracker |
| 60 | libra2-native-interface | creditchain-native-interface |
| 61 | libra2-release-builder | creditchain-release-builder |
| 62 | libra2-resource-viewer | creditchain-resource-viewer |
| 63 | libra2-sdk-builder | creditchain-sdk-builder |
| 64 | libra2-transaction-benchmarks | creditchain-transaction-benchmarks |
| 65 | libra2-transaction-simulation | creditchain-transaction-simulation |
| 66 | libra2-transactional-test-harness | creditchain-transactional-test-harness |
| 67 | libra2-validator-interface | creditchain-validator-interface |
| 68 | libra2-vm | creditchain-vm |
| 69 | libra2-vm-benchmarks | creditchain-vm-benchmarks |
| 70 | libra2-vm-environment | creditchain-vm-environment |
| 71 | libra2-vm-logging | creditchain-vm-logging |
| 72 | libra2-vm-profiling | creditchain-vm-profiling |
| 73 | libra2-vm-types | creditchain-vm-types |
| 74 | libra2-workspace-server | creditchain-workspace-server |
| 75 | libra2-framework | creditchain-framework |
| 76 | libra2-stdlib | creditchain-stdlib |
| 77 | libra2-token | creditchain-token |
| 78 | libra2-token-objects | creditchain-token-objects |

### 3.3 Node & Infrastructure

| # | Old | New |
|---|-----|-----|
| 79 | libra2-node | creditchain-node |
| 80 | libra2-cargo-cli | creditchain-cargo-cli |
| 81 | libra2-move-debugger (in libra2-move/) | creditchain-move-debugger |
| 82 | libra2-e2e-comparison-testing | creditchain-e2e-comparison-testing |

### 3.4 Directory Renames

| Old Path | New Path |
|----------|----------|
| libra2-move/ | creditchain-move/ |
| libra2-node/ | creditchain-node/ |
| .assets/libra2_banner.png | .assets/creditchain_banner.png |

---

## 4. Automated Rebranding Script

### 4.1 rename_crates.sh

```bash
#!/bin/bash
# CreditChain Rebranding Script
# Phase 2: Crate name replacement
# MUST be run from repository root

set -euo pipefail

echo "=== CreditChain Rebranding: Phase 2 ==="
echo "This will rename all libra2 references to creditchain"
echo ""

# Step 1: Replace in all Cargo.toml files
echo "[1/6] Updating Cargo.toml files..."
find . -name "Cargo.toml" -not -path "./target/*" -exec \
    sed -i '' 's/libra2-/creditchain-/g' {} +
find . -name "Cargo.toml" -not -path "./target/*" -exec \
    sed -i '' 's/name = "libra2"/name = "creditchain"/g' {} +

# Step 2: Replace in all Rust source files
echo "[2/6] Updating Rust source files..."
find . -name "*.rs" -not -path "./target/*" -exec \
    sed -i '' 's/libra2_/creditchain_/g' {} +
find . -name "*.rs" -not -path "./target/*" -exec \
    sed -i '' 's/libra2-/creditchain-/g' {} +
find . -name "*.rs" -not -path "./target/*" -exec \
    sed -i '' 's/Libra2/CreditChain/g' {} +
find . -name "*.rs" -not -path "./target/*" -exec \
    sed -i '' 's/LIBRA2/CREDITCHAIN/g' {} +

# Step 3: Replace in Move files
echo "[3/6] Updating Move files..."
find . -name "*.move" -not -path "./target/*" -exec \
    sed -i '' 's/libra2_/creditchain_/g' {} +
find . -name "*.move" -not -path "./target/*" -exec \
    sed -i '' 's/Libra2/CreditChain/g' {} +

# Step 4: Replace in config/YAML/TOML files
echo "[4/6] Updating config files..."
find . \( -name "*.yaml" -o -name "*.yml" -o -name "*.toml" -o -name "*.json" \) \
    -not -path "./target/*" -exec \
    sed -i '' 's/libra2/creditchain/g' {} +

# Step 5: Rename directories
echo "[5/6] Renaming directories..."
if [ -d "libra2-move" ]; then
    mv libra2-move creditchain-move
fi
if [ -d "libra2-node" ]; then
    mv libra2-node creditchain-node
fi
# Rename crate directories
for dir in crates/libra2-*; do
    if [ -d "$dir" ]; then
        newdir="${dir/libra2/creditchain}"
        mv "$dir" "$newdir"
    fi
done

# Step 6: Update workspace Cargo.toml paths
echo "[6/6] Updating workspace member paths..."
sed -i '' 's|"libra2-move/|"creditchain-move/|g' Cargo.toml
sed -i '' 's|"libra2-node"|"creditchain-node"|g' Cargo.toml
sed -i '' 's|"crates/libra2-|"crates/creditchain-|g' Cargo.toml
sed -i '' 's|"crates/libra2"|"crates/creditchain"|g' Cargo.toml

echo ""
echo "=== Rebranding complete! ==="
echo "Next steps:"
echo "  1. cargo check  (verify compilation)"
echo "  2. cargo test    (verify tests pass)"
echo "  3. Review git diff for any missed references"
echo "  4. Update Move.toml named addresses"
```

### 4.2 Verification Script

```bash
#!/bin/bash
# verify_rebranding.sh — Check for remaining libra2 references

echo "=== Checking for remaining 'libra2' references ==="

# Count remaining references (excluding .git, target, Cargo.lock)
count=$(grep -rl "libra2" --include="*.rs" --include="*.toml" \
    --include="*.yaml" --include="*.yml" --include="*.move" \
    --include="*.md" --exclude-dir=.git --exclude-dir=target \
    . 2>/dev/null | wc -l)

if [ "$count" -gt 0 ]; then
    echo "WARNING: Found $count files still containing 'libra2'"
    echo "Files:"
    grep -rl "libra2" --include="*.rs" --include="*.toml" \
        --include="*.yaml" --include="*.yml" --include="*.move" \
        --include="*.md" --exclude-dir=.git --exclude-dir=target .
else
    echo "SUCCESS: No remaining 'libra2' references found!"
fi

echo ""
echo "=== Checking cargo build ==="
cargo check 2>&1 | tail -5
```

---

## 5. Config File Changes

### 5.1 Node Configuration

**Old: `libra2-node.yaml`**
```yaml
base:
  role: "validator"
  data_dir: "/opt/libra2/data"
  waypoint:
    from_config: "0:genesis_hash"
```

**New: `creditchain-node.yaml`**
```yaml
base:
  role: "validator"
  data_dir: "/opt/creditchain/data"
  waypoint:
    from_config: "0:genesis_hash"
```

### 5.2 Environment Variables

| Old | New |
|-----|-----|
| LIBRA2_NODE_CONFIG | CREDITCHAIN_NODE_CONFIG |
| LIBRA2_DATA_DIR | CREDITCHAIN_DATA_DIR |
| LIBRA2_LOG_LEVEL | CREDITCHAIN_LOG_LEVEL |
| LIBRA2_METRICS_PORT | CREDITCHAIN_METRICS_PORT |
| LIBRA2_API_PORT | CREDITCHAIN_API_PORT |

### 5.3 Docker Updates

**Old Dockerfile references:**
```dockerfile
FROM libra2/libra2-node:latest
COPY libra2-node.yaml /opt/libra2/etc/
ENTRYPOINT ["/usr/local/bin/libra2-node"]
```

**New:**
```dockerfile
FROM ibankio/creditchain-node:latest
COPY creditchain-node.yaml /opt/creditchain/etc/
ENTRYPOINT ["/usr/local/bin/creditchain-node"]
```

---

## 6. Move Framework Rebranding

### 6.1 Named Addresses

Update all `Move.toml` files:

```toml
# Old
[addresses]
libra2_framework = "0x1"
libra2_std = "0x1"
libra2_token = "0x3"

# New
[addresses]
creditchain_framework = "0x1"
creditchain_std = "0x1"
creditchain_token = "0x3"
```

### 6.2 Module Names

| Old Module | New Module | Address |
|-----------|-----------|---------|
| libra2_framework::libra2_account | creditchain_framework::creditchain_account | 0x1 |
| libra2_framework::libra2_coin | creditchain_framework::creditchain_coin | 0x1 |
| libra2_framework::libra2_governance | creditchain_framework::creditchain_governance | 0x1 |
| libra2_std::* | creditchain_std::* | 0x1 |
| libra2_token::token | creditchain_token::token | 0x3 |

---

## 7. CI/CD Updates

### 7.1 GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CreditChain CI

on:
  push:
    branches: [main]
  pull_request:

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build CreditChain
        run: cargo build --release
      - name: Test CreditChain
        run: cargo test
      - name: Build Docker Image
        run: docker build -t ibankio/creditchain-node:${{ github.sha }} .
```

### 7.2 Docker Compose

```yaml
# docker/docker-compose.yaml
services:
  creditchain-node-0:
    image: ibankio/creditchain-node:latest
    container_name: creditchain-validator-0
    volumes:
      - ./creditchain-node-0.yaml:/opt/creditchain/etc/creditchain-node.yaml
    command: ["/usr/local/bin/creditchain-node", "-f", "/opt/creditchain/etc/creditchain-node.yaml"]
    ports:
      - "8080:8080"   # API
      - "6182:6182"   # Validator network
      - "9101:9101"   # Metrics
```

---

## 8. Rebranding Checklist

- [ ] README.md updated with CreditChain branding
- [ ] Banner image replaced (.assets/creditchain_banner.png)
- [ ] All Cargo.toml package names updated (200+ files)
- [ ] All Rust source `use libra2_*` → `use creditchain_*`
- [ ] All Rust source `Libra2` → `CreditChain` in strings/docs
- [ ] All Move source files updated
- [ ] All Move.toml named addresses updated
- [ ] Config files renamed and content updated
- [ ] Docker files updated
- [ ] CI/CD pipelines updated
- [ ] Environment variable prefixes updated
- [ ] Binary names updated
- [ ] Directory names updated (libra2-move → creditchain-move)
- [ ] Workspace Cargo.toml member paths updated
- [ ] `cargo check` passes
- [ ] `cargo test` passes
- [ ] Docker build succeeds
- [ ] No remaining "libra2" references (verified by script)
- [ ] CODEOWNERS updated
- [ ] LICENSE headers updated
- [ ] Git tags updated

---

*CreditChain Rebranding — Every Reference, Every File, Every Line*
