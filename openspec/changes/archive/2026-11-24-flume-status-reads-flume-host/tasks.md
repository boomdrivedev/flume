# Tasks

Each task names the narrowest check that shows it done. Design sections
are this change's `design.md`.

## 1. sluice's records on sluice host's socket

- [x] 1.1 Add `crates/flume/src/profiles/merge_queue/sluice/ask.rs`: one ask of sluice host's socket, and the records of each base kept with their version; read-submission asks `overview` and `status <base> --all` through it (design 4). Check: `sluice exec main -- moon run flume:test`.
- [x] 1.2 Pin sluice 0.110.0 in `devenv.nix`. Check: `sluice exec main -- moon run flume:tools`.

## 2. flume host's socket

- [x] 2.1 Add `crates/flume/src/served.rs`: the socket's path, its server of `status` and `subscribe`, its client, and the events of flume's state folder (design 1 to 3, 6). Check: `sluice exec main -- moon run flume:test`.
- [x] 2.2 `flume host` binds the socket, serves it beside its watch, moves its subscriptions after each settle, and removes the socket when it stops. Check: `sluice exec main -- moon run flume:test`.
- [x] 2.3 `flume status` and the MCP tool `status` ask `flume host`, refused where none answers (design 5). Check: `sluice exec main -- moon run flume:test`.

## 3. The tests

- [x] 3.1 Add `crates/flume/tests/common/resident.rs`, and run the resident in each harness that asks status, as a host's service manager runs it. Check: `sluice exec main -- moon run flume:test`.
- [x] 3.2 Run `sluice daemon` in the tools layer's harnesses that read sluice, and the tests' own git with no repository hook. Check: `sluice exec main -- moon run flume:tools`.

## 4. The rule and the site

- [x] 4.1 Amend core.15 on `core.md` and `core-details.md`. Check: `sluice exec main -- moon run root:format`.
- [x] 4.2 Say on the site's commands page that `flume status` asks `flume host`, and give `flume host` and its socket. Check: `sluice exec main -- moon run site:build`.
