# Design

## 1. The two writes

The claim stays a compare-and-set write before the call, so two
settles never submit the place twice (core.16, core.17). It clears
`due`, sets `submitting`, keeps the entry before as `before`, and clears
the request, the verdict and the wait, as before. It does not count.

The answer write sets `submitting` to false, adds 1 to `submits`, and
records the entry, the request or the refusal. A refusal gives its
verdict and its wait in this write. `verdict_at` gives a refusal the
same verdict and wait through one function, `refuse`.

## 2. A stop during the call

A command that stops during the call leaves `submitting` true and
`submits` not counted. Status gives `submitting`, so the caller sees
that flume called the merge queue and has no answer. The line, the
telemetry and the wait on the submission read `submitting` as a submit
(`SubmissionHeld::made`), so a stop during the call reads as it did
before.

## 3. The test

The fake store runs a hook just before each write. The scenario arms a
hook that reads the place and arms itself again. Thus it reads each
state that another command can read between two writes. Against the
code before this change, the scenario fails on the first state after
the claim: `submits` 1 with no entry, request or verdict.
