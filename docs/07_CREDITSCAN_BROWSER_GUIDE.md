# CreditScan Browser Guide

> Document 07 | CreditChain Product Guide | Version 1.0
> Scope: Creditscan browser, network configuration, indexer requirements, production deployment

---

## 1. Overview

Creditscan is the official blockchain browser for CreditChain. It is the main
user-facing surface for observing network activity, browsing accounts, reading
transactions, inspecting validators, and exposing analytics for operators and
support teams.

Creditscan is not a standalone product. It depends on the CreditChain fullnode
and, for richer account and history views, the indexer and GraphQL stack.

## 2. Product Responsibilities

Creditscan provides:

- Account, block, transaction, validator, and analytics pages
- Mainnet, testnet, devnet, and local network switching
- Explorer views backed by REST plus indexer-backed GraphQL data
- Operational visibility for support and ecosystem teams

For the docs site and marketing site, Creditscan should be described as the
**production explorer for CreditChain**.

## 3. Local Project Source

The implementation currently lives in:

`/Users/wenyan/Libra2Projects/creditscan`

Key files:

- `README.md`
- `package.json`
- `src/global-config/network-selection.ts`
- `src/themes/colors/creditchainColorPalette.ts`

## 4. Runtime Configuration

Creditscan supports network endpoint configuration through environment variables.

| Variable | Default |
|----------|---------|
| `CREDITCHAIN_NODE_REST_URL` | `https://rpc.creditchain.org/v1` |
| `CREDITCHAIN_INDEXER_URL` | `https://indexer.creditchain.org` |
| `CREDITCHAIN_TESTNET_URL` | `https://testnet.creditchain.org/v1` |
| `CREDITCHAIN_DEVNET_URL` | `https://devnet.creditchain.org/v1` |
| `CREDITCHAIN_LOCAL_URL` | `http://127.0.0.1:8080/v1` |
| `CREDITCHAIN_LOCALNET_URL` | `http://127.0.0.1:8080/v1` |

Network selection can be changed from the browser UI or by appending:

```text
?network=mainnet
?network=testnet
?network=devnet
?network=local
```

## 5. Production Topology

Creditscan can render basic data from the node REST API, but production explorer
behavior requires a fuller indexed data path.

### Publicly reachable services

- CreditChain fullnode REST API
- Indexer HTTP / GraphQL endpoint
- Creditscan frontend

### Internal services

- gRPC manager and data service
- Indexer processors
- Postgres
- Any analytics refresh workers

If the indexer stack is missing, Creditscan can still load basic chain data but
account tabs such as transaction history, fungible assets, or richer analytics
may show empty states.

## 6. Recommended Production Checklist

1. Publish a stable mainnet RPC URL.
2. Publish an indexer HTTP endpoint that fronts the indexed dataset.
3. Ensure processors continuously write chain data into Postgres.
4. Validate Creditscan against mainnet, testnet, devnet, and local modes.
5. Add monitoring for RPC latency, GraphQL health, and browser asset delivery.
6. Version the frontend together with any breaking changes to the indexer schema.

## 7. Local Development

```bash
pnpm install
cp .env.example .env.local
pnpm start
```

Default local URL:

`http://localhost:3000`

## 8. Docs Portal Use

The docs site should use this guide to answer:

- What is Creditscan?
- Which services are required in production?
- How does network selection work?
- Which environment variables should operators set?
- When is indexer / GraphQL required?

The main website should use Creditscan as one of the three top-level product
surfaces alongside CreditChain Core and the TypeScript SDK.
