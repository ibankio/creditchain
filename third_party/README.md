# CreditChain Third-Party Dependencies

The `third_party/` tree contains synchronized copies of selected upstream
projects that are critical to the CreditChain stack, most notably Move-related
code. These mirrors allow CreditChain to make atomic, reviewable changes across
system boundaries while keeping security-critical dependencies under direct
source control.

## Why This Exists

- security-sensitive dependencies can be reviewed and pinned in-tree
- protocol and dependency changes can land together when required
- builds remain reproducible without depending on moving upstream targets

## Developer Rules

- respect upstream project boundaries and architecture
- do not introduce dependencies from mirrored crates back into unrelated
  CreditChain-specific code unless the upstream project could accept that shape
- keep changes partitioned and documented so they can be reasoned about later

In practice, most contributors can work in this tree through normal repository
changes. Sync mechanics are handled separately.

## Sync Model

Administrative syncs with upstream repositories are typically performed through
Copybara-based workflows. Those flows should preserve commit metadata and keep
the mirrored project independently buildable.

## Related Docs

- [`../README.md`](../README.md)
- [`../docs/03_MOVE_MODULES_SPEC.md`](../docs/03_MOVE_MODULES_SPEC.md)
