# CreditChain Rust SDK

The `sdk/` workspace contains the Rust SDK for CreditChain. It is the native
developer surface inside this repository for applications, services, and tools
that need to read chain state, build transactions, sign payloads, and submit
operations from Rust.

## Scope

Key modules include:

- `client`: REST client and network access helpers
- `crypto`: signing and verification primitives
- `transaction_builder`: helpers for constructing transactions
- `types`: on-chain data structures and transaction types
- `examples/`: sample Rust integrations

## Positioning

This README documents the Rust SDK that ships inside the main protocol
repository.

For JavaScript and TypeScript applications, the official public SDK is
`creditchain-ts-sdk` in the separate SDK repository. Its public-facing guidance
is captured in
[`../docs/08_TYPESCRIPT_SDK_GUIDE.md`](../docs/08_TYPESCRIPT_SDK_GUIDE.md).

## Typical Use Cases

- backend services that need direct Rust integration
- tooling for operators and automation
- transaction builders, signers, and local network utilities
- integration tests and performance tooling

## Related Docs

- [`../api/README.md`](../api/README.md)
- [`../docs/08_TYPESCRIPT_SDK_GUIDE.md`](../docs/08_TYPESCRIPT_SDK_GUIDE.md)
- [`../README.md`](../README.md)
