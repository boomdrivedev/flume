# A settle reads the host's deliveries once

## Why

On deneb the runner's `flume --json settle --every 60s` held a whole
core: each pass took longer than its 60-second bound, so the watch never
rested. The host's deliveries are one document, 1.3 MB holding 5,088 of
weir's records, and every place's settle read it whole and worked out
what it owes again at each act that asks: each place's batches, its
ends, its wakes, its status line and each hand-over. A pass over 55
pending places read and parsed the document thousands of times, and each
read ran weir's `owed` over every record. Over a copy of deneb's state
a pass took 186 to 198 seconds.

## What Changes

- **An engine holds the host's deliveries it last read or wrote (43,
  75, C.7).** It reads the document whole only once its version moved:
  each ask first reads the version alone, and reads the records again
  only where another command wrote them since. What they owe is worked
  out once for each version and each second of the host's clock. A
  write of the engine's own holds what it wrote at the version the write
  gave, so the act after it reads nothing again. Every write is still by
  compare-and-set: a write at a version that moved reads again and
  answers over what stands, as before (75; core.17).
- **The store seam gives a host document's version alone (153).** One
  operation beside `read_host`, by default the version `read_host`
  gives. The state directory reads it from the document's opening
  bytes, since it writes a document's version as its first field, and
  reads the document whole where it opens otherwise. The fakes' store
  logs the read apart from a read of the document.
- **A conformance scenario pins the cost.** A watch's settle over three
  pending places reads the host's deliveries whole once, the next settle
  with nothing written since reads them none, and one after another
  command confirms an end reads them once.

Nothing a person or an agent reads changes: what is owed, handed over,
confirmed and let go of is what it was, and every surface answers as
before. A done delivery of a place whose document stands is kept, as
the deliveries spec holds (62; weir 107, I9).

## Impact

- `crates/flume-seams/src/store.rs`: `Store::host_version`.
- `crates/flume/src/profiles/store/state_dir/mod.rs`: the state
  directory's `host_version`.
- `crates/flume-fakes/src/store.rs`: the fakes' `host_version`, logged.
- `crates/flume-core/src/deliveries.rs`, `lib.rs`, `batch.rs`,
  `message.rs`: the engine's held reading, and every read and write of
  the host's deliveries through it.
- `crates/flume-conformance`: the cost scenario, and the store seam's
  version read in the host documents' scenario.
