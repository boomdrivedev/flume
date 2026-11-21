# sluice's documents at 44e33d4

The documents a stand-in `sluice` answers the merge-queue profile with in
`merge_queue_sluice_documents.rs`, each written from sluice's code at
`44e33d4` on `bolt/sluice-is-refactored-and-runs-on-pools`, where sluice
holds a landing the forge refused (sluice 152; sluice record 0124). The
tree pins sluice 0.93.2, which holds no landing, so these are written
from the code rather than captured from a run of a build the tree does
not pin (core.22). A field flume does not read is kept to the shape the
code prints; its value is made up.

| file | answers | written from, at `44e33d4` |
|---|---|---|
| `overview.json` | `sluice overview --json`, naming `main` and `train/x` | `document_held` in `crates/sluice/src/overview.rs`, each branch a `BranchView` of `crates/sluice-core/src/overview.rs` |
| `status-main.json` | `sluice status main --all --json`: the place's `fix/one` entry #7, refused, and the base place's `train/x` entry #9 by `from.head`, refused, each with no `refused`; and an earlier withdrawn #5 of `fix/one` | `status_json` in `crates/sluice/src/lib.rs`, each record `decided_document(sub, None)`, which gives a refused record no `notice`, since `Submission::decided` gives none for `State::Refused`, and no `refused` |
| `status-main-refused-words.json` | the same, each refused record carrying `refused {words, at}` | `report` in `crates/sluice/src/lib.rs`, which adds `refused` from `landing_refusal` |
| `status-train-x.json` | `sluice status train/x --all --json`, its queue empty | `status_json` |
| `inbox-refused.json` | `sluice inbox --json`: #6 landed under `6:landed` and #7 refused under `7:refused`, each owed | `inbox` in `crates/sluice/src/lib.rs`, each outcome `decided_document(sub, Some(decided))` with its `delivery` as `OwedOutcome::left("inbox", at, false)` of `crates/sluice-core/src/owed.rs` gives it; #7's `notice` as `sluice_core::notice` words `Decided::Refused` |
| `inbox-parked.json` | `sluice inbox --json`: #6 landed under `6:landed` and #8 in the state `parked`, which sluice has no word for at `44e33d4`, under `8:parked` | as `inbox-refused.json`, standing for a state a later sluice may add |
| `confirm.json` | `sluice confirm <key> --json` | `confirm` in `crates/sluice/src/lib.rs`, its `confirmed` document |
