# TypeScript SDK Guide

> Document 08 | CreditChain Product Guide | Version 1.0
> Scope: creditchain-ts-sdk installation, configuration, workflows, docs positioning

---

## 1. Overview

`creditchain-ts-sdk` is the official TypeScript SDK for interacting with the
CreditChain blockchain. It gives application developers a typed interface for:

- network-aware clients
- account management
- transaction building and submission
- fungible assets and on-chain data access
- custom endpoint configuration for mainnet, testnet, devnet, and local nodes

The SDK should be presented on the docs site and the marketing site as the
**primary developer entrypoint for JavaScript and TypeScript applications**.

## 2. Local Project Source

The implementation currently lives in:

`/Users/wenyan/Libra2Projects/creditchain-ts-sdk`

Important files:

- `README.md`
- `package.json`
- `src/api/creditchain.ts`
- `src/api/creditchainConfig.ts`
- `src/index.ts`
- `examples/`

## 3. Installation

```bash
pnpm install creditchain-ts-sdk
```

The package name is `creditchain-ts-sdk`.

## 4. Core Entry Points

The SDK exports:

- `CreditChain`
- `CreditChainConfig`
- `Network`
- account classes and transaction helpers

Typical setup:

```ts
import { CreditChain, CreditChainConfig, Network } from "creditchain-ts-sdk";

const config = new CreditChainConfig({ network: Network.TESTNET });
const client = new CreditChain(config);
```

## 5. Common Workflows

### Read data

- account resources
- modules
- transactions
- balances
- fungible assets

### Submit transactions

1. build
2. simulate
3. sign
4. submit
5. wait for execution

### Override endpoints

Use `CreditChainConfig` to point applications at custom:

- fullnode
- faucet
- indexer
- pepper
- prover

This is important for enterprise environments, custom deployments, and local
integration testing.

## 6. Examples

The SDK repository already includes examples for:

- simple transfers
- local node workflows
- sponsored transactions
- fungible assets
- multisig
- custom clients
- key management

These examples should be mirrored or linked from the docs portal.

## 7. Docs Portal Use

The docs site should break SDK content into:

1. install
2. configure
3. fetch data
4. build and submit transactions
5. local development
6. examples and recipes

The website should describe `creditchain-ts-sdk` as the SDK that matches the
production chain and explorer release train.

## 8. Suggested Website Copy

Use these short summaries where needed:

- "Typed client tooling for transactions, assets, accounts, and custom endpoints."
- "CreditChain and CreditChainConfig entrypoints for network-aware clients."
- "Build, simulate, sign, and submit transactions with production-ready examples."

## 9. Build And Docs

Useful scripts in the SDK project:

```bash
pnpm build
pnpm doc
pnpm test
```

The docs portal should treat the SDK repository as the source of truth for API
shape and examples, while keeping high-level integration guides in the main
CreditChain docs site.
