# CreditChain Testsuite

The `testsuite/` workspace contains the main integration, resilience, replay,
and performance testing harnesses for CreditChain.

## Directory Map

- `forge/`: cluster orchestration and end-to-end network testing
- `forge-cli/`: CLI entrypoints for Forge workflows
- `smoke-test/`: high-signal regression coverage for common node flows
- `loadtest-k6/`: load generation and performance scenarios
- `fuzzer/`: fuzzing entrypoints
- `replay-verify/`: historical replay verification utilities
- `benchmark-workloads/`: reusable benchmark definitions
- `test_framework/`, `testcases/`, `verify_core/`: shared test logic and suites

## What This Covers

- local devnet validation
- large-scale cluster testing
- performance and throughput measurement
- regression testing for protocol, API, and operational changes
- replay-based confidence checks for ledger correctness

## Practical Notes

If Docker tooling fails on Apple Silicon with an error like:

```text
no match for platform in manifest: not found
```

run the affected workflow with:

```bash
DOCKER_DEFAULT_PLATFORM=linux/amd64 ...
```

## Related Docs

- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../docs/07_CREDITSCAN_BROWSER_GUIDE.md`](../docs/07_CREDITSCAN_BROWSER_GUIDE.md)
