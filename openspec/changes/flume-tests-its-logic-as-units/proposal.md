# flume tests its logic as units

## Why

flume has 1,259 tests. 299 of them are in files that start a real
binary. Most of those tests start flume's binary, or the test binary as
flume, on scratch repositories with real git. Many of them also start
`flume host`, herdr or Rex servers, sluice and a stand-in for Claude
Code. The same behaviour is often proven twice: once by a conformance
scenario over the fakes, and again through the binary.

On 2026-11-27, the person ruled on the same shape in sluice: "that is
way too many fucking tests. and terrible design", and "fix it. fix the
fucking testing disaster." The ruling holds for each repository with
that shape, flume among them.

## What Changes

- **Logic is tested as a unit.** A unit test is in the crate that holds
  the logic. It drives the logic through its seams and fakes:
  `flume-fakes` and the conformance world of `flume-conformance`. Where
  time matters, it runs on a paused clock. A unit test starts no
  process, no git and no network.
- **A small end-to-end set runs the real binary.** One test proves the
  main path of each command. One test proves each seam with a real
  tool. The tools are git, the pinned openspec, sluice, herdr, Rex, the
  host's files and processes, and a collector of telemetry. A few
  walkthroughs prove a place from the `launch` command to its landing. The set holds about 40 tests
  in the gate. Each end-to-end test owns each process that it starts
  and ends it. The set starts a bounded number of processes at one
  time.
- **A test that duplicates the proof of another test goes.** Each
  behaviour that a test proves today stays proven by exactly one test.
  `moves.md` in this change gives each test that moves or goes, and the
  test that proves its behaviour after the move.
- **The gate gets faster and lighter.** The gate's test time and the
  peak count of flume and git processes during the suite fall. This
  change gives both counts, measured on deneb before and after.

The change makes no mode that only the tests use, and no transition
code. A profile that runs its tool takes the call to its tool as a
value. The binary gives it the call that starts the tool's process, and
a unit test gives it a call that answers from a recording. The
end-to-end test of that seam proves the recordings against the pinned
tool.

## Measured on deneb

| | Before | After |
|---|---|---|
| Tests | 1,259 | (after the last step) |
| Tests in files that start a real binary | 299 | (after the last step) |
| Wall time of `flume:test` | (to measure) | (after the last step) |
| Wall time of `flume:tools` | (to measure) | (after the last step) |
| Peak count of flume processes | (to measure) | (after the last step) |
| Peak count of git processes | (to measure) | (after the last step) |

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `the-gate`: the tests of logic are units, and a small end-to-end set
  runs the real binary.

## Impact

- `crates/flume/tests/`: each file shrinks to its end-to-end tests, or
  goes.
- `crates/flume/src/`: the unit tests of the commands, the printed
  words and the profiles.
- `crates/flume-conformance/src/engine/`: a scenario for each behaviour
  of the core that only a binary test proved.
- `crates/flume/src/profiles/`: the herdr, Rex, sluice, openspec,
  Ghostty and Terminal profiles take the call to their tool as a
  value.
- Boom: the change rests on the invariant
  `logic-is-tested-as-units-and-few-tests-run-the-binary@3131d0f94b83`,
  which binds the whole system.
