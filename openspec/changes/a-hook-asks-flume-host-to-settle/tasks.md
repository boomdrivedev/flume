# Tasks

Each task names the narrowest check that shows it done.

## 1. flume host takes the settle

- [x] 1.1 Add the `settle` ask to flume host's socket, and a client that sends it. flume host answers one `taken` line. Then it settles on a task that it tracks, and reads the agent's quiet on its own clock. Check: `sluice exec main -- moon run flume:test`.

## 2. The hook asks flume host

- [x] 2.1 The Stop hook and the start in another mode send the ask to flume host, and the detached start goes. A test shows that the hook leaves no process when it ends. Check: `sluice exec main -- moon run flume:test`.

## 3. The site

- [x] 3.1 The Claude Code guide and the commands page say that the Stop hook asks flume host to settle. Check: `sluice exec main -- moon run site:build`.
