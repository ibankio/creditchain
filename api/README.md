# CreditChain API

The `api/` workspace exposes the public data plane for CreditChain nodes. It is
the primary interface used by wallets, services, Creditscan, and SDK clients to
read chain state, submit transactions, and inspect network health.

## What Lives Here

- `src/`: REST handlers, routing, and request/response logic
- `types/`: shared API-facing data structures
- `doc/`: generated OpenAPI spec artifacts and rendered documentation
- `openapi-spec-generator/`: tooling used to generate the canonical spec files

## Responsibilities

- expose the `/v1` REST surface for accounts, transactions, events, tables, and metadata
- publish health and readiness endpoints for operators
- generate the OpenAPI contract used by docs and downstream client tooling
- provide a stable public interface for Creditscan and external integrations

Creditscan uses the node REST API for basic chain views such as ledger metadata,
blocks, transactions, and account-level reads. Richer historical and analytics
views still require the indexer stack described in
[`docs/07_CREDITSCAN_BROWSER_GUIDE.md`](../docs/07_CREDITSCAN_BROWSER_GUIDE.md).

## Change Workflow

If the API changes, update the implementation first and then regenerate the
specification files from the repository root:

```bash
cargo run -p creditchain-openapi-spec-generator -- -f yaml -o api/doc/spec.yaml
cargo run -p creditchain-openapi-spec-generator -- -f json -o api/doc/spec.json
```

To preview the rendered spec locally:

```bash
cd api
make serve
```

Then open `http://127.0.0.1:8888/spec.html`.

## Production Notes

- Public REST traffic should terminate on fullnodes, not directly on validators.
- API versioning must stay aligned with SDK and explorer release expectations.
- Breaking response changes should be reflected in docs and communicated with the
  Creditscan and `creditchain-ts-sdk` maintainers.

## Related Docs

- [`../docs/01_CREDITCHAIN_ARCHITECTURE.md`](../docs/01_CREDITCHAIN_ARCHITECTURE.md)
- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../docs/07_CREDITSCAN_BROWSER_GUIDE.md`](../docs/07_CREDITSCAN_BROWSER_GUIDE.md)
