# The tools layer waits only for what it reads

## Why

Main's gate requires `flume:tools`, `cargo test -p flume --features
tools --test tools`, 183 tests, and it no longer finishes inside the
gate's ten minutes. Timed on an idle deneb at ccb715e9 it took 271
seconds of wall time and 2,238 seconds of test time, and under the gate,
beside the other tasks, it was cut off at 125 tests. Most of that time
was waiting that reads nothing:

- The herdr profile's 25 terminal scenarios ran one after another in
  one test, each on a herdr server of its own, 199 seconds, the longest
  test of the run.
- Every agent the herdr tests opened waited three seconds for a stand-in
  that never draws to draw, after herdr's own three-second settle, and a
  start held at a question waited ten seconds for a folder-trust
  question the stand-in draws within one.
- The scenario of a session at work read what the session took with a
  ten-second wait for input before it sent anything, where none was due.
- The scenario of a program that never comes up waited out the harness's
  twenty-second start bound, where herdr takes three.
- The seven tests of the words in the pinned Claude Code each read the
  241 MB program and scanned it byte by byte against every fragment, 25
  to 55 seconds each.
- Rex's every-key test typed about two thousand keys and separators one
  Rex call at a time into one block, 77 seconds.

## What Changes

- **A profile's scenarios run side by side.** The conformance suite
  gains a runner that takes a profile's scenarios a few at a time, each
  over a fresh harness of its own, and names every failure in the
  scenarios' order. The herdr and Rex profiles' terminal scenarios run
  on it, five at a time, each harness on a server and in a scratch of
  its own.
- **A harness waits only for what a scenario awaits.** The terminal
  harness's reading of what a recipient's session took names how many
  inputs it awaits, reading at once where none is, and a harness can
  give the program that never comes up a seam with the shortest start
  bound its terminal takes.
- **The herdr profile takes the reading of the folder-trust question it
  is given.** A draw and a settle of the profile's own replace the three
  and ten seconds, for a program known to draw at once. flume's own
  starts keep three and ten; the herdr tests give the stand-in's, no
  draw and four seconds.
- **The pinned Claude Code is read once.** Its words' tests share one
  read of the program and find each fragment with `memchr`'s `memmem`.
- **Rex's keys are typed into four blocks side by side**, each its own
  `cat` and its share of the keys, every step's assertion as before.

No scenario, assertion or clause changes what it holds.
