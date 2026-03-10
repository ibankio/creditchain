# CreditChain Protobufs

The `protos/` workspace defines the shared protobuf schema used across
CreditChain services and generated clients. It is the contract layer for gRPC
and other protobuf-backed integrations.

## Directory Map

- `proto/`: canonical `.proto` source files
- `rust/`: generated Rust bindings
- `typescript/`: generated TypeScript bindings
- `python/`: generated Python bindings
- `scripts/`: installation and generation helpers

## Responsibilities

- keep shared wire formats versioned in one place
- generate language bindings for supported SDKs and services
- support indexer, streaming, and service-to-service APIs

This matters for public-facing surfaces because Creditscan, indexer components,
and external tooling often depend on consistent generated types.

## Update Workflow

Install dependencies:

```bash
cd protos
./scripts/install_deps.sh
```

Regenerate all supported bindings:

```bash
cd protos
./scripts/build_protos.sh
```

If you use `buf` locally, keep the version aligned with CI.

## Related Docs

- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../docs/07_CREDITSCAN_BROWSER_GUIDE.md`](../docs/07_CREDITSCAN_BROWSER_GUIDE.md)
