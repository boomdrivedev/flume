# Tasks

The work is on `fix/main--a-quiet-delivered-stage-ends-whatever-ran-after`,
cut from `main`. The statement is read at boomdrivedev/blueprints bf9e16f
(40, 46, 74, 133, the glossary's quiet).

## 1. The settle

- [x] 1.1 Have the submission's step count its collect a change only where the place's held version moved (40, 46, 74, 133). True when `sluice exec main -- moon run conformance:test` passes, with `engine/settling.rs`'s `an_outcome_handed_again_holds_no_stage` failing before the change and passing after.

## 2. The spec

- [x] 2.1 Validate with `openspec validate an-outcome-handed-again-holds-no-stage --strict`, then archive with `openspec archive an-outcome-handed-again-holds-no-stage --yes`.
