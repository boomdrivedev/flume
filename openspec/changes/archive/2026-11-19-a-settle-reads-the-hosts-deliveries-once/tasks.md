# Tasks

## 1. The store seam's version read

- [x] 1.1 In `crates/flume-seams/src/store.rs`, give the `Store` trait `fn host_version(&self, key: &HostKey) -> Option<u64>`, by default the version `read_host` gives. In `crates/flume-fakes/src/store.rs`, give the fakes' store its own, logging `host_version <key>`. In `crates/flume/src/profiles/store/state_dir/mod.rs`, read it from the document's opening bytes, the document read whole where they open otherwise, held by `a_documents_version_is_read_from_the_bytes_it_opens_with` and `a_document_that_opens_otherwise_gives_no_version_from_its_head`. Check: `devenv shell -- cargo test -p flume --lib state_dir`.
- [x] 1.2 In `crates/flume-conformance/src/seams/store.rs`, have the host documents' scenario read each document's version alone, none before a write and after a removal and the written version from another process, the seam-contracts spec's "A host document's version read alone". Check: `devenv shell -- cargo test -p flume --test profiles store`.

## 2. The engine reads the host's deliveries once for each version

- [x] 2.1 In `crates/flume-core/src/deliveries.rs` and `lib.rs`, give the engine its held reading of the host's deliveries, `Engine::deliveries`, `Engine::owing` and `Engine::owing_of` over it, and `Engine::rewrite_deliveries_whole`, `rewrite_deliveries` and `add_deliveries`, which hold what they wrote; route every read and write of the host's deliveries in `deliveries.rs`, `batch.rs` and `message.rs` through them. Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [x] 2.2 In `crates/flume-conformance/src/engine/end_delivery.rs`, add `a_watch_reads_the_deliveries_once`, the deliveries spec's "A watch's settle over three pending places". Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
