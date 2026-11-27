# A submit is counted with its answer

## Why

flume wrote a submit in two writes. The first write counted the submit
and cleared the answer to the submit before it. Then flume called the
merge queue. The second write recorded the answer: an entry, a request
or a refusal. A refusal came in a third write. Thus a reader between the
writes found a submit counted with no answer.

`flume:tools` showed this on deneb on 2026-11-27. The walkthrough
`a_pull_requests_failure_goes_back_to_code_once_and_its_next_request_lands`
waits until `submits` is 2, and then expects the request. Under load,
its status read came between the two writes and found `submits` 2 and
no `request`. The test failed 3 times when the host was busy and passed
5 times when it was not. The pinned sluice and its JSON had no part in
it: the walkthrough's stand-in for sluice gives the answer.

## What Changes

- **The claim marks the place submitting, with no count.** Before
  flume calls the merge queue, it writes `submitting` and no count, so
  a command that stops during the call leaves the place known as
  submitting.
- **One write counts the submit with its answer.** The write that
  records the merge queue's answer adds 1 to `submits` and clears
  `submitting`. A refusal is in that write too: its verdict and the
  words that the place waits on.
- **Status gives `submitting`.** The form only gains the field
  (core.14). The place's line, its telemetry and the wait on its
  submission read a place that is submitting as a place that flume
  submitted.
- **A conformance scenario reads the place before each write.** For a
  request, an entry and a refusal, it fails where a reader finds a
  submit counted without its answer.

No clause changes (48, 111, 124, core.16).
