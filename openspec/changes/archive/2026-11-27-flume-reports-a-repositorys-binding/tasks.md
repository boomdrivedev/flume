# Tasks

Each task names the narrowest check that shows it done. Design sections
are this change's `design.md`.

## 1. The seams and the fakes

- [x] 1.1 Add `BindingHeld`, `HeldFrom` and `Kinds::read_binding_held` to the kinds seam, and `LandsOn` and `MergeQueue::lands_on` to the merge-queue seam, each with a default that refuses (design 2, 3). Check: `sluice exec main -- moon run seams:test`.
- [x] 1.2 The fake kinds and the fake merge queue answer the new operations. Check: `sluice exec main -- moon run fakes:test`.

## 2. The report

- [x] 2.1 Add `crates/flume-core/src/repository.rs`: the merge queue's readiness, each kind's `lands` and `refused`, and `repositories` in `Answer::Status` (design 1, 4, 5). Check: `sluice exec main -- moon run core:test`.
- [x] 2.2 The weir profile reads the binding held, and the sluice profile answers lands-on (design 2, 3). Check: `sluice exec main -- moon run flume:test`.
- [x] 2.3 `flume host`'s ask takes `repositories`. `flume status --binding` and the MCP tool's `binding` name the repository, and the printed answer gives it (design 1). Check: `sluice exec main -- moon run flume:test`.
- [x] 2.4 The conformance suite's full status answer lists the new fields. Check: `sluice exec main -- moon run conformance:test`.

## 3. The skill

- [x] 3.1 Add `plugins/claude/skills/setting-up-flume/SKILL.md`, and write it into the plugin of the agents that the binary starts (design 6). Check: `sluice exec main -- moon run plugin-claude:validate`.

## 4. The site

- [x] 4.1 Give `flume status --binding` and its JSON on the site's commands page. Check: `sluice exec main -- moon run site:build`.
