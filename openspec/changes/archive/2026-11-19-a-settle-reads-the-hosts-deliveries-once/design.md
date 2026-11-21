# Design: a settle reads the host's deliveries once

The statement's clauses this rests on: 43 (an end is one of weir's
deliveries), 75 (a write at a moved version reads again), 153 (the
host's own documents), 225 (a watch's settle reads only the pending
places) and C.7 (the deliveries document). The rules: core.15, core.16,
core.17 and core.62 on `docs/architecture/core.md`. No area page
covers `crates/flume-core/src/deliveries.rs` or the store profile.

## 1. The engine's reading of the host's deliveries

The engine keeps, beside its other state, the host's deliveries whole as
it last read or wrote them, with their version, and what they owed at a
second of the host's clock. `Engine::deliveries` reads the version
alone through the store seam and gives the reading held while the
version is the one it holds; otherwise it reads the document whole and
holds it. `Engine::owing` gives what the reading owes now, worked out
once per reading and second, shared rather than copied.

Every write goes through `Engine::rewrite_deliveries_whole`, which asks
its caller's function over the reading as it stands and writes the
result at that version; once the write lands it holds what it wrote at
the new version. A refused write reads the version again, finds it
moved, reads the document whole and asks again, as `rewrite_whole` did
(75; core.17).

The reading is never a record of flume's: it lives in the engine of one
command, is checked against the store's version at every ask, and goes
with the command (core.15, core.16). A command reads what another
command wrote at its next ask, as before.

## 2. The store seam's version read

`Store::host_version` gives the version a host document stands at,
none where there is none, by default from `read_host`. The state
directory writes each host document with its version as its first
field, so it reads the version from the first 128 bytes, `{"version":`
and a whole number, and reads the document whole for its version where
the bytes open otherwise. The fakes' store logs `host_version <key>`
apart from `read_host <key>`, so a test counts the reads of a whole
document.

## 3. What stays

A done delivery is let go of only once no document of its place stands
(62; weir 107, I9): a delivery let go of while its place's document
stands would be recorded again from the document's ends, and handed
again. So the settle keeps those records, and the cost is held by
reading the document once rather than by holding fewer records.

weir's change `owed-reads-each-record-once` makes `owed` read each
record once; flume takes it when a chore moves its pin to a weir main
that holds it, and this change holds the settle's cost apart from it.
