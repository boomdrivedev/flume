# Tasks

## 1. The inbox

- [x] 1.1 Ask sluice host's socket for a place's inbox, as `sluice inbox --json` in the place asks it, with its `SLUICE_AGENT` as the caller, and start no sluice process while it answers. True when `devenv shell -- cargo test -p flume --lib the_inbox_is_asked_of_sluice_host_on_its_socket_and_starts_no_sluice` passes.
- [x] 1.2 Where no sluice host answers, run `sluice inbox --json` in the place. True when `devenv shell -- cargo test -p flume --lib the_inbox_runs_sluice_inbox_where_no_sluice_host_answers` passes.

## 2. The stand-ins

- [ ] 2.1 Have the documents' stand-in sluice host answer `inbox` from its fixture. True when `sluice exec <base> -- moon run flume:test` passes.
- [ ] 2.2 Have the stand-in host of a base that takes pull requests answer `inbox` with no outcomes, and the walk of a sluice host started again by a settle's inbox hold. True when `sluice exec <base> -- moon run flume:tools` passes.

## 3. The specs

- [ ] 3.1 Archive this change so the specs reconcile (core.19). True when `sluice exec <base> -- moon run root:specs` passes.
