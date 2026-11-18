---
number: 80
title: flume hands no work to a sluice too old to merge it by itself
status: proposed
date: 2026-11-18
decision-makers:
- Chuck Swanberg
- the construct stage of the change flume-says-when-sluice-is-too-old-to-merge-its-work
---

# flume hands no work to a sluice too old to merge it by itself

Status: proposed.
sluice 0.97.0 and later merge your work on their own once flume hands it over; flume no longer pushes sluice along. An older sluice takes the work and then never merges it, and nothing says why. So before flume hands sluice any work, it asks sluice which version it is. If sluice is older than 0.97.0, or gives a version that is no release, flume keeps the work back. The work then waits for you with a message naming the sluice you have and saying flume needs 0.97.0 or later. Once sluice is updated, asking flume to try the work again sends it on. Say yes if this is what you meant.

Deciders: the launch's brief, in its words: "Flume never leaves work waiting on a sluice too old to merge it by itself: it refuses to hand that sluice the work and says which sluice version it needs." The construct stage of the change `flume-says-when-sluice-is-too-old-to-merge-its-work`, on the rest: where the check runs, how sluice's version is read, the release flume needs, and a sluice whose version names no release. Each is made on the rules written and is cheap to change.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 111, 112, 124, 203 and C.6's submit and version rows, as `5efb363` amends them on the branch `design/flume-says-when-sluice-is-too-old-to-merge-its-work`, which waits on the orchestrator's push; sluice 135 and sluice record 0093, its host process; sluice record 0111, under which a build of sluice's main says 0.0.0, which no release has; sluice's tags `v0.96.2`, whose `submit --no-wait` returns with nothing started to move the queue, and `v0.97.0`, the first to hold `crates/sluice/src/host_process`, read on 2026-11-18; the change `a-settle-runs-no-round-of-the-merge-queue`; rules core.1, core.2 and core.6.

## Context and problem statement

The change `a-settle-runs-no-round-of-the-merge-queue` made the sluice profile's step run nothing. From sluice 0.97.0, sluice's own host process moves each queue: the submit starts it, and a settle's read starts it again where work waits and none runs (124, 125, C.6; sluice 135). A sluice before 0.97.0 has no host process. Its `sluice submit --no-wait` adds the entry and returns, and only a `sluice step`, a `sluice run` or a daemon moves the queue. flume runs none of them now. So on a host whose sluice is older, a submitted place waits for ever, shown submitted and undecided, with nothing that says why. On 2026-11-18 the login `PATH` of the host deneb found a sluice that said `sluice 0.95.1`.

## Considered options

For where flume checks:

1. **At each submit, in the sluice profile, before `sluice submit` (chosen).**
2. **At the launch of a kind that lands**, refused whole as core.8 refuses a launch asking for more than flume runs.
3. **Only the site's word** that flume needs sluice 0.97.0 or later.

For how flume reads what the sluice can do:

1. **`sluice --version`, compared with the release flume needs (chosen).**
2. **A probe for the host process's own command**, `sluice host --help`.

For a sluice whose version names no release, 0.0.0 from a build of sluice's main, a pre-release, or no answer:

1. **Refused as an older one is (chosen).**
2. **Taken as new enough.**

For which release flume needs:

1. **The first release whose host process moves its queues, 0.97.0, a constant of the sluice profile (chosen).**
2. **The release the tree pins**, read from `devenv.nix`.

## Decision outcome

- **Before each submit**, a place's from its worktree and a base place's by name, the sluice profile runs `sluice --version` in the directory and environment the submit runs in.
- **It hands sluice the work only from 0.97.0 on.** The answer's first line must be `sluice <major>.<minor>.<patch>`, each part a whole number, compared part by part, so 0.100.0 is later than 0.97.0. Any later release, of any major version, is handed the work.
- **Otherwise it runs no `sluice submit`** and answers the submit refused. Its words name what sluice said, or why it said nothing, and that flume needs sluice 0.97.0 or later, the first release whose own process moves its queues.
- **The core holds that as any refused submission.** The place waits on its caller's retry or drop, status naming why (112), and its stall reaches its recipients (130). A retry once sluice is updated submits the place as any retry does. The seam, the core and the fakes are unchanged.
- **Nothing else is checked.** The forecast submits nothing, and open, close, equip and the reads hand sluice no work to merge.

Why the paths not taken were not taken:

- **At the launch.** A launch's stages may run for hours, and the host's sluice may be updated meanwhile. A launch refused for it would need the merge-queue seam to grow a check the core calls, for one profile's floor (core.1, core.2). Refused at the submit, the work stays committed in its place, lost to nothing.
- **The site's word alone.** The work would still wait unsaid on a host that missed the page.
- **A probe.** The brief asks flume to say the version it needs, and `--version` is one answer every release gives alike. A probe ties flume to a command of sluice's that flume never runs.
- **A build that names no release taken.** A build of sluice's main may be from before the host process, and flume cannot tell. The brief asks that work never wait on such a sluice. A host that builds sluice from source can install a release.
- **The pinned release.** The pin moves with every chore that updates it, and a host whose sluice is older than the pin but has the host process merges flume's work. The brief's floor is a sluice that merges "by itself".

### Consequences

- Rule core.6's details rule out a `sluice submit` to such a sluice. Statement C.6's submit row and its new version row say the same, on blueprints `5efb363`.
- Each submit runs one more sluice command, `sluice --version`.
- A host whose sluice is older sees each place it submits wait on its caller with the words, until its sluice is updated and the submission retried.
