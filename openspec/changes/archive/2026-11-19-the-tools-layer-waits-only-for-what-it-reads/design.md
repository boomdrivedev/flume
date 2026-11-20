# Design

## Context

See proposal.md, Why. A herdr agent start costs about three seconds of
herdr's own settle before it answers, which flume cannot shorten. What
flume's tests spend past that is theirs to remove.

## Decisions

### 1. Scenarios side by side, a bounded few at a time

`flume_conformance::scenario::pass_all_side_by_side(of, scenarios,
at_once, fresh)` starts `at_once` workers in a thread scope; each takes
the next scenario's index from one counter, makes a fresh harness in its
own thread and runs the scenario there, so a harness need not be `Send`.
Failures are sorted back into the scenarios' order and reported in the
words `pass_all` uses, which both now share. A panic in a worker is
raised again in the test. Five at a time keeps 25 herdr servers from
starting at once on a gate host already running other tests.

### 2. A harness waits only for what is awaited

`TerminalHarness::received(session, awaited)` gives what the session
took once it took `awaited` inputs, or by the harness's wait where it
takes fewer, and reads at once where `awaited` is none. The scenario of
a session at work reads what was taken before the send at once, and
after the refused send awaits one more input for the harness's wait, as
its check that nothing was typed always did. `never_up_seam` defaults
to the harness's seam; herdr's gives herdr's shortest start bound,
3,001 milliseconds, and Rex's four seconds, as the profiles' own
never-up tests do.

### 3. The trust reading is a profile setting

`Herdr::with_trust_reading(draw, settle)` replaces `TRUST_DRAW` and
`TRUST_SETTLE` in `settled`. Nothing in flume's own commands sets it, so
every start flume makes reads as before. The herdr tests give no draw,
since the stand-in draws whatever it draws before it tells herdr it is
ready and herdr answers only after its settle, and a settle of four
seconds, room for the question the stand-in draws a second after it
says it is held, on a loaded host.

### 4. The pinned program read once

The words' tests keep the program, its version and its bytes in one
`OnceLock`, and each fragment is found by `memchr::memmem::find`, a
dev-dependency of flume at the version the lock already holds.

### 5. Rex's keys over four blocks

The every-key test opens four blocks, each in a place of its own and
running `cat`, splits the key names into four shares, and types each
share, then the keys the relay does not hand Rex by Rex's own
`send-key`, into its block in a thread of its own; the first block also
types the three keys around a refused one. The steps and what each
`cat` read are joined in order before any assertion, and every
assertion reads them as before. Each block's relay must still read its
screen once every key was typed.

## What stays

Each walkthrough through the built binary under herdr still pays
herdr's settle and flume's three-second draw per agent it opens, since
the binary takes no test setting; those are the costs of flume's own
starts.
