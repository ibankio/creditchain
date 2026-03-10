# CreditChain Docs

This directory is the canonical content source for:

- `https://docs.creditchain.org`
- `https://www.creditchain.org/docs`
- the main site in `/Users/wenyan/ClaudeProjects/creditchain-web`

It covers the full CreditChain product system:

1. **CreditChain Core**: the Layer 1 protocol, token, architecture, modules, and operations
2. **Creditscan**: the blockchain browser and its production deployment requirements
3. **creditchain-ts-sdk**: the developer SDK and common integration workflows
4. **Website and docs content**: canonical positioning, IA, and reusable summaries

## Recommended Reading Paths

### For operators

1. `01_CREDITCHAIN_ARCHITECTURE.md`
2. `05_DEPLOYMENT_AND_OPERATIONS.md`
3. `07_CREDITSCAN_BROWSER_GUIDE.md`
4. top-level component READMEs such as `api/README.md`, `network/README.md`, and `storage/README.md`

### For application developers

1. `01_CREDITCHAIN_ARCHITECTURE.md`
2. `03_MOVE_MODULES_SPEC.md`
3. `06_ONE_CLICK_STABLECOIN.md`
4. `08_TYPESCRIPT_SDK_GUIDE.md`

### For product, docs, and website teams

1. `README.md` at the repository root
2. `07_CREDITSCAN_BROWSER_GUIDE.md`
3. `08_TYPESCRIPT_SDK_GUIDE.md`
4. `09_WEBSITE_AND_DOCS_CONTENT_MAP.md`

## Document Index

| File | Purpose |
|------|---------|
| `01_CREDITCHAIN_ARCHITECTURE.md` | System architecture and strategic position |
| `02_TOKEN_ECONOMY_REFERENCE.md` | Reference token economy and genesis parameters |
| `03_MOVE_MODULES_SPEC.md` | CreditChain-specific Move modules |
| `04_BRIDGE_AND_INTEROP_SPEC.md` | Bridge strategy and interoperability design |
| `05_DEPLOYMENT_AND_OPERATIONS.md` | Node deployment, infrastructure, and observability |
| `06_ONE_CLICK_STABLECOIN.md` | Stablecoin factory and platform design |
| `07_CREDITSCAN_BROWSER_GUIDE.md` | Creditscan product and production topology |
| `08_TYPESCRIPT_SDK_GUIDE.md` | SDK installation, configuration, and workflows |
| `09_WEBSITE_AND_DOCS_CONTENT_MAP.md` | Canonical copy map for the site and docs portal |

## Content Rules

- Root README files should stay concise and reusable as source material for the docs site.
- Product names should be written consistently as **CreditChain**, **Creditscan**, and **creditchain-ts-sdk**.
- Token economy numbers in `02_TOKEN_ECONOMY_REFERENCE.md` remain illustrative and non-binding unless governance explicitly publishes a binding issuance plan.
- Website copy should always align with the messaging and section mapping in `09_WEBSITE_AND_DOCS_CONTENT_MAP.md`.
