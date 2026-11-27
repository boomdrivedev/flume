## ADDED Requirements

### Requirement: Whether sluice lands a base, read with no fetch

The sluice profile SHALL say whether sluice lands a base, with no fetch
and no sluice command. It SHALL read `.config/sluice.toml` at the base
and at each remote's copy of it. Where no revision holds the policy, its
words SHALL name `.config/sluice.toml`. Where the policy names no table
for the base under `[branches]`, its words SHALL say so (126, core.15).

#### Scenario: No policy, then a policy for another branch, then one for main

- **WHEN** status names a repository that binds sluice, first with no `.config/sluice.toml`, then with one for another branch only, then with one for `main`
- **THEN** the merge queue's words name `.config/sluice.toml` at `main`, then say that it names no table for `main`, and then the merge queue is ready
