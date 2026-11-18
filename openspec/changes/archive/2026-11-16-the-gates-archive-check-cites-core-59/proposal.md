# The gate's archive check cites core.59

## Why

The bolt `bolt/flumes-gate-checks-changes-as-archived` wrote record
0072 and rule core.58 for `root:changes`, which archives each open
change alone and holds every spec it writes to `openspec validate
--strict`. main landed record 0072, flume confirming a merge queue's
outcome once it has acted on it, and rule core.58, a submission whose
landing the forge refused, first. `root:adrs` fails two records of one
number, so the bolt's record is 0073 and its rule core.59, and the-gate's
requirement "Each open change is validated as it would archive" still
cites core.58, which now names the forge's refusal (core.19).

## What Changes

- **The-gate's requirement cites core.59.** "Each open change is
  validated as it would archive" ends `(core.59)`; its words and its
  scenarios are as they were.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: "Each open change is validated as it would archive" cites
  its rule as core.59.
