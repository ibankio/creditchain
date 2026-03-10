# CreditChain Dashboards

The `dashboards/` directory contains Grafana dashboards used to operate
CreditChain infrastructure. These dashboards support validator operators,
fullnode operators, SRE teams, and explorer support workflows.

## Contents

- dashboard JSON source files
- gzipped binary variants used when packaging dashboards into Kubernetes
  ConfigMaps

## Update Workflow

Edit the `.json` source files, then regenerate the compressed artifacts:

```bash
cd dashboards
gzip -fkn *.json
```

## Deployment Notes

- These files are intended to be mounted or synced into Grafana deployments.
- Compressed variants are used to stay within ConfigMap size limits.
- Dashboard changes should track major node, API, indexer, or Creditscan
  operational changes.

## Related Docs

- [`../docs/05_DEPLOYMENT_AND_OPERATIONS.md`](../docs/05_DEPLOYMENT_AND_OPERATIONS.md)
- [`../api/README.md`](../api/README.md)
- [`../storage/README.md`](../storage/README.md)
