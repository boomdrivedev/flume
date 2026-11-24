# Tasks

Each task names the narrowest check that shows it done. Design sections
are this change's `design.md`.

## 1. What is followed

- [x] 1.1 In `crates/flume-core`, add `Engine::followed` and its reading of held state, with the settling scenario of a chore followed while its submission is in flight (design 3). Check: `sluice exec main -- moon run conformance:test`.

## 2. The resident

- [x] 2.1 Add `crates/flume/src/resident.rs`: the loop, the settle on the blocking pool, the lock and `host`, each with its tests over a paused clock (design 1, 2, 5, 6). Check: `sluice exec main -- moon run flume:test`.
- [x] 2.2 Add the sluice profile's `Subscription`, with its tests against a stand-in sluice host on a socket (design 4). Check: `sluice exec main -- moon run flume:test`.
- [x] 2.3 Add `flume host --every <bound>`, refused with no positive bound. Check: `sluice exec main -- moon run flume:test`.

## 3. The service

- [x] 3.1 Declare `flume-host` in `nix/modules/darwin.nix` and `nix/modules/nixos.nix`, and their tests (design 7). Check: `sluice exec main -- moon run nix:test`.

## 4. The rule

- [x] 4.1 Write record 0082, and amend core.13 on `core.md` and `core-details.md`. Check: `sluice exec main -- moon run root:format`.
