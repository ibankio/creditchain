# CreditChain Deployment & Operations Guide

> Document 05 | CreditChain Design Series | Version 1.0
> Scope: Node deployment, network operations, monitoring, enterprise deployment models

---

## 1. Deployment Models

CreditChain is institution-grade infrastructure. Deployment model depends on
the operator's regulatory requirements, privacy needs, and scale.

### 1.1 Deployment Matrix

| Model | Validators | Access | Governance | Target Operator |
|-------|-----------|--------|------------|-----------------|
| **Public Network** | Open staking (50-500) | Public API | On-chain voting | CreditChain Foundation |
| **Consortium** | Invited institutions (7-21) | Permissioned | Multi-party agreement | Banking consortiums |
| **Private Enterprise** | Single org (4-7 nodes) | Fully private | Internal policy | Central banks, corporates |
| **Hybrid** | Core + satellite | Layered access | Federated | Multi-national banks |
| **Sovereign** | Government-operated | National access | Regulatory | Central bank CBDC |

### 1.2 Licensing

CreditChain is proprietary enterprise infrastructure. Deployment requires
licensing agreement with CreditChain Research Team / iBank.

| License Tier | Nodes | Support | Features |
|-------------|-------|---------|----------|
| Foundation | Unlimited | Community | Public network participation |
| Enterprise | Up to 50 | 24/7 SLA | Private deployment, custom genesis |
| Sovereign | Unlimited | Dedicated team | Full customization, regulatory adaptation |

---

## 2. Hardware Requirements

### 2.1 Validator Node

| Resource | Minimum | Recommended | Enterprise |
|----------|---------|-------------|-----------|
| CPU | 16 cores | 32 cores | 64 cores |
| RAM | 64 GB | 128 GB | 256 GB |
| Storage | 2 TB NVMe | 4 TB NVMe | 8 TB NVMe RAID |
| Network | 1 Gbps | 10 Gbps | 10 Gbps dedicated |
| OS | Ubuntu 22.04+ | Ubuntu 22.04+ | RHEL 9 / Ubuntu 22.04 |

### 2.2 Full Node (API Serving)

| Resource | Minimum | Recommended |
|----------|---------|-------------|
| CPU | 8 cores | 16 cores |
| RAM | 32 GB | 64 GB |
| Storage | 1 TB NVMe | 2 TB NVMe |
| Network | 1 Gbps | 5 Gbps |

### 2.3 Archive Node

| Resource | Minimum | Notes |
|----------|---------|-------|
| CPU | 16 cores | History processing |
| RAM | 64 GB | Index building |
| Storage | 10 TB+ NVMe | Full chain history |
| Network | 5 Gbps | Indexer data streaming |

---

## 3. Node Deployment

### 3.1 Single Validator Setup

```bash
# 1. Install CreditChain binary
curl -sSf https://releases.creditchain.org/install.sh | bash
# Or build from source:
git clone https://github.com/ibankio/creditchain.git
cd creditchain
cargo build --release -p creditchain-node

# 2. Generate validator keys
creditchain keygen generate-keys \
    --output-dir /opt/creditchain/keys \
    --key-type ed25519

# 3. Initialize node configuration
creditchain node init \
    --config-dir /opt/creditchain/etc \
    --data-dir /opt/creditchain/data \
    --chain-id 52225 \
    --role validator

# 4. Configure node
cat > /opt/creditchain/etc/creditchain-node.yaml << 'EOF'
base:
  role: "validator"
  data_dir: "/opt/creditchain/data"
  waypoint:
    from_file: "/opt/creditchain/etc/waypoint.txt"

consensus:
  safety_rules:
    service:
      type: "local"
    backend:
      type: "on_disk_storage"
      path: "/opt/creditchain/data/safety-rules.db"

execution:
  genesis_file_location: "/opt/creditchain/etc/genesis.blob"

validator_network:
  listen_address: "/ip4/0.0.0.0/tcp/6182"
  mutual_authentication: true
  identity:
    type: "from_file"
    path: "/opt/creditchain/keys/validator-identity.yaml"

full_node_networks:
  - network_id: "public"
    listen_address: "/ip4/0.0.0.0/tcp/6183"
    identity:
      type: "from_file"
      path: "/opt/creditchain/keys/fullnode-identity.yaml"

api:
  enabled: true
  address: "0.0.0.0:8080"

storage:
  rocksdb_configs:
    max_open_files: 10000
    max_background_jobs: 16

state_sync:
  state_sync_driver:
    bootstrapping_mode: "execute_or_apply_from_genesis"
    continuous_syncing_mode: "execute_transactions"
EOF

# 5. Start node
creditchain-node \
    --config /opt/creditchain/etc/creditchain-node.yaml \
    2>&1 | tee /var/log/creditchain/node.log
```

### 3.2 Systemd Service

```ini
# /etc/systemd/system/creditchain-node.service
[Unit]
Description=CreditChain Validator Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=creditchain
Group=creditchain
ExecStart=/usr/local/bin/creditchain-node \
    --config /opt/creditchain/etc/creditchain-node.yaml
Restart=always
RestartSec=10
LimitNOFILE=1000000
LimitNPROC=1000000

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/creditchain/data /var/log/creditchain
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

### 3.3 Docker Deployment

```yaml
# docker-compose.yaml — CreditChain 4-Validator Devnet
version: "3.8"

x-creditchain-common: &cc-common
  image: ibankio/creditchain-node:latest
  restart: unless-stopped
  ulimits:
    nofile:
      soft: 1000000
      hard: 1000000

services:
  cc-validator-0:
    <<: *cc-common
    container_name: cc-validator-0
    volumes:
      - ./configs/validator-0.yaml:/opt/creditchain/etc/creditchain-node.yaml
      - ./genesis.blob:/opt/creditchain/etc/genesis.blob
      - ./keys/validator-0:/opt/creditchain/keys
      - cc-data-0:/opt/creditchain/data
    ports:
      - "8080:8080"   # API
      - "6182:6182"   # Validator network
      - "9101:9101"   # Metrics
    networks:
      - cc-network

  cc-validator-1:
    <<: *cc-common
    container_name: cc-validator-1
    volumes:
      - ./configs/validator-1.yaml:/opt/creditchain/etc/creditchain-node.yaml
      - ./genesis.blob:/opt/creditchain/etc/genesis.blob
      - ./keys/validator-1:/opt/creditchain/keys
      - cc-data-1:/opt/creditchain/data
    ports:
      - "8081:8080"
      - "6183:6182"
      - "9102:9101"
    networks:
      - cc-network

  cc-validator-2:
    <<: *cc-common
    container_name: cc-validator-2
    volumes:
      - ./configs/validator-2.yaml:/opt/creditchain/etc/creditchain-node.yaml
      - ./genesis.blob:/opt/creditchain/etc/genesis.blob
      - ./keys/validator-2:/opt/creditchain/keys
      - cc-data-2:/opt/creditchain/data
    ports:
      - "8082:8080"
      - "6184:6182"
      - "9103:9101"
    networks:
      - cc-network

  cc-validator-3:
    <<: *cc-common
    container_name: cc-validator-3
    volumes:
      - ./configs/validator-3.yaml:/opt/creditchain/etc/creditchain-node.yaml
      - ./genesis.blob:/opt/creditchain/etc/genesis.blob
      - ./keys/validator-3:/opt/creditchain/keys
      - cc-data-3:/opt/creditchain/data
    ports:
      - "8083:8080"
      - "6185:6182"
      - "9104:9101"
    networks:
      - cc-network

  # Monitoring stack
  prometheus:
    image: prom/prometheus:latest
    container_name: cc-prometheus
    volumes:
      - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    ports:
      - "9090:9090"
    networks:
      - cc-network

  grafana:
    image: grafana/grafana:latest
    container_name: cc-grafana
    volumes:
      - ./monitoring/grafana:/var/lib/grafana
      - ./monitoring/dashboards:/etc/grafana/provisioning/dashboards
    ports:
      - "3000:3000"
    networks:
      - cc-network

volumes:
  cc-data-0:
  cc-data-1:
  cc-data-2:
  cc-data-3:

networks:
  cc-network:
    driver: bridge
```

---

## 4. Network Operations

### 4.1 Local Devnet (Single Machine)

```bash
# Quick start: 4-validator local devnet
creditchain localnet run \
    --num-validators 4 \
    --chain-id 52227 \
    --with-faucet \
    --faucet-port 8081 \
    --api-port 8080

# Verify devnet is running
curl http://localhost:8080/v1/-/healthy
# Expected: {"message":"creditchain-node:ok"}

# Get ledger info
curl http://localhost:8080/v1/ | jq .
```

### 4.2 Testnet Joining

```bash
# 1. Download testnet genesis
curl -o genesis.blob https://genesis.creditchain.org/testnet/genesis.blob
curl -o waypoint.txt https://genesis.creditchain.org/testnet/waypoint.txt

# 2. Verify genesis hash
sha256sum genesis.blob
# Expected: <published hash from CreditChain team>

# 3. Configure for testnet
creditchain node init \
    --config-dir /opt/creditchain/etc \
    --chain-id 52226 \
    --role full_node

# 4. Start full node
creditchain-node --config /opt/creditchain/etc/creditchain-node.yaml

# 5. Check sync status
curl http://localhost:8080/v1/ | jq '.ledger_version, .ledger_timestamp'
```

### 4.3 Validator Operations

```bash
# Join validator set (requires stake)
creditchain stake add-stake \
    --profile mainnet \
    --amount 1000000000000000  # 10M CCC in Octas

# Check validator status
creditchain node show-validator-set

# Rotate consensus key (recommended every epoch)
creditchain node rotate-key --key-type consensus

# Leave validator set gracefully
creditchain stake unlock-stake --amount <full_amount>
# Wait 30 epochs (lockup period)
creditchain stake withdraw-stake
```

---

## 5. Monitoring & Observability

### 5.1 Metrics (Prometheus)

CreditChain exposes 500+ Prometheus metrics on port 9101.

Key metrics:

| Metric | Type | Description |
|--------|------|-------------|
| `cc_consensus_proposals_count` | Counter | Blocks proposed |
| `cc_consensus_round` | Gauge | Current consensus round |
| `cc_consensus_last_committed_round` | Gauge | Last committed round |
| `cc_execution_txns_per_block` | Histogram | Transactions per block |
| `cc_storage_latest_version` | Gauge | Latest ledger version |
| `cc_mempool_size` | Gauge | Pending transaction count |
| `cc_api_requests_total` | Counter | API request count |
| `cc_api_request_duration` | Histogram | API latency |
| `cc_state_sync_version` | Gauge | State sync progress |
| `cc_network_peers` | Gauge | Connected peer count |

### 5.2 Prometheus Configuration

```yaml
# monitoring/prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'creditchain-validators'
    static_configs:
      - targets:
        - 'cc-validator-0:9101'
        - 'cc-validator-1:9101'
        - 'cc-validator-2:9101'
        - 'cc-validator-3:9101'
    metrics_path: '/metrics'
```

### 5.3 Grafana Dashboards

| Dashboard | Panels | Purpose |
|-----------|--------|---------|
| Consensus Health | Round progress, proposal rate, vote latency | Validator performance |
| Execution | TPS, block time, execution latency | Throughput monitoring |
| Storage | State size, DB operations, compaction | Disk usage |
| Network | Peer count, bandwidth, message rates | Connectivity |
| API | Request rate, latency percentiles, errors | Service quality |
| IUSD | Supply, mint/burn rate, reserve ratio | Stablecoin health |
| Bridge | Volume, pending transfers, operator status | Cross-chain ops |

### 5.4 Alerting Rules

```yaml
# monitoring/alerts.yml
groups:
  - name: creditchain-critical
    rules:
      - alert: ConsensusStalled
        expr: rate(cc_consensus_proposals_count[5m]) == 0
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "Consensus has stalled - no new proposals in 2 minutes"

      - alert: HighMempool
        expr: cc_mempool_size > 100000
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "Mempool size exceeding 100K transactions"

      - alert: LowPeerCount
        expr: cc_network_peers < 3
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Validator has fewer than 3 peers"

      - alert: StorageNearFull
        expr: node_filesystem_avail_bytes{mountpoint="/opt/creditchain/data"} < 50e9
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Less than 50GB storage remaining"

      - alert: IUSDReserveRatioLow
        expr: cc_iusd_reserve_ratio_bps < 10000
        for: 1m
        labels:
          severity: critical
        annotations:
          summary: "IUSD reserve ratio below 100%"

      - alert: BridgeOperatorDown
        expr: cc_bridge_active_operators < cc_bridge_threshold
        for: 10m
        labels:
          severity: critical
        annotations:
          summary: "Insufficient bridge operators for threshold"
```

---

## 6. Backup & Recovery

### 6.1 State Snapshots

```bash
# Create state snapshot (hot backup)
creditchain node backup \
    --backup-dir /opt/creditchain/backups \
    --state-version latest

# Restore from snapshot
creditchain node restore \
    --backup-dir /opt/creditchain/backups/snapshot-12345 \
    --data-dir /opt/creditchain/data

# Verify restored state
creditchain node verify-state \
    --data-dir /opt/creditchain/data
```

### 6.2 Disaster Recovery

| Scenario | Recovery |
|----------|----------|
| Single validator down | Restart, state sync catches up |
| Data corruption | Restore from snapshot + state sync |
| Multiple validators down | Reconstruct from archive nodes |
| Full network partition | Consensus halts, resumes when partition heals |
| Key compromise | Emergency key rotation via governance |

### 6.3 Backup Schedule

| Backup Type | Frequency | Retention | Storage |
|-------------|-----------|-----------|---------|
| State snapshot | Every 6 hours | 7 days | S3/GCS |
| Full archive | Daily | 30 days | Cold storage |
| Key backup | On rotation | Indefinite | HSM/Vault |
| Config backup | On change | 90 days | Git |

---

## 7. Enterprise Deployment: Private Chain

### 7.1 Custom Genesis for Private Deployment

```bash
# Generate private chain genesis
creditchain genesis generate-private \
    --chain-id 52228 \
    --chain-name "ACME-CreditChain" \
    --num-validators 4 \
    --ccc-supply 100000000 \
    --iusd-issuer "0xACME_ISSUER" \
    --output /opt/creditchain/private/

# This generates:
# - genesis.blob (private genesis)
# - waypoint.txt
# - validator-keys/ (4 key sets)
# - creditchain-node-*.yaml (4 configs)
```

### 7.2 Private Chain Configuration

```yaml
# Private chain specific settings
base:
  role: "validator"

# Disable public-facing features
api:
  enabled: true
  address: "10.0.0.1:8080"  # Internal network only

# Permissioned networking
validator_network:
  mutual_authentication: true
  # Only connect to known validators
  seeds:
    cc-validator-0: { addresses: ["10.0.0.10:6182"] }
    cc-validator-1: { addresses: ["10.0.0.11:6182"] }

# Disable external full node connections
full_node_networks: []

# Access control
access_control:
  transaction_submission: "permissioned"
  authorized_submitters:
    - "0xACME_APP_1"
    - "0xACME_APP_2"
```

---

## 8. Security Hardening

### 8.1 Node Security

| Control | Implementation |
|---------|---------------|
| Network isolation | Validator nodes on private VLAN |
| Firewall rules | Only 6182 (validator), 8080 (API) exposed |
| Key management | HSM for consensus keys (YubiHSM/CloudHSM) |
| OS hardening | CIS Level 2 benchmark |
| Disk encryption | LUKS full-disk encryption |
| Log protection | Immutable audit logs to SIEM |
| Access control | MFA + role-based SSH access |

### 8.2 Key Storage

| Key Type | Storage | Rotation |
|----------|---------|----------|
| Consensus key | HSM | Every epoch |
| Network key | Encrypted file | Quarterly |
| Account key | Cold wallet / HSM | As needed |
| API keys | Vault (HashiCorp) | Monthly |

---

## 9. Upgrade Procedures

### 9.1 Node Binary Upgrade

```bash
# 1. Download new binary
curl -O https://releases.creditchain.org/v1.2.0/creditchain-node

# 2. Verify signature
gpg --verify creditchain-node.sig creditchain-node

# 3. Rolling upgrade (one validator at a time)
systemctl stop creditchain-node
cp creditchain-node /usr/local/bin/
systemctl start creditchain-node

# 4. Verify new version
curl http://localhost:8080/v1/-/healthy
```

### 9.2 Framework Upgrade (Governance)

```bash
# 1. Propose framework upgrade
creditchain governance propose-framework-upgrade \
    --framework-path creditchain-move/framework/build \
    --metadata "Upgrade to v2.0: Add IUSD module"

# 2. Validators vote
creditchain governance vote \
    --proposal-id 42 \
    --vote yes

# 3. Auto-execute at epoch boundary when quorum reached
# Framework modules updated on-chain
```

---

## 10. Operational Runbook

### 10.1 Common Operations

| Operation | Command |
|-----------|---------|
| Check node health | `curl localhost:8080/v1/-/healthy` |
| Get current epoch | `curl localhost:8080/v1/ \| jq .epoch` |
| Check sync status | `curl localhost:8080/v1/ \| jq .ledger_version` |
| View validator set | `creditchain node show-validator-set` |
| Check peer connections | `curl localhost:9101/metrics \| grep cc_network_peers` |
| Tail logs | `journalctl -u creditchain-node -f` |
| Restart node | `systemctl restart creditchain-node` |

### 10.2 Troubleshooting

| Symptom | Cause | Resolution |
|---------|-------|------------|
| Node not syncing | Peer connectivity | Check firewall, DNS, seed peers |
| Consensus timeout | Network latency | Check inter-validator RTT |
| High memory usage | State cache growth | Tune RocksDB cache settings |
| Disk full | Chain growth | Enable pruning or expand storage |
| API slow | Load spike | Scale API nodes horizontally |

---

## 11. Invariants

| ID | Invariant |
|----|-----------|
| OPS-1 | Validator MUST have fresh consensus key (rotated within current epoch) |
| OPS-2 | State snapshots MUST be taken every 6 hours minimum |
| OPS-3 | Private chain MUST NOT expose validator network port publicly |
| OPS-4 | HSM MUST be used for consensus keys in production |
| OPS-5 | Binary upgrades MUST be rolling (one validator at a time) |
| OPS-6 | Monitoring MUST alert within 2 minutes of consensus stall |
| OPS-7 | All operator actions MUST be logged to immutable audit trail |

---

*CreditChain Operations — Enterprise-Grade Infrastructure for AI-Era Finance*
