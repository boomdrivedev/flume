## ADDED Requirements

### Requirement: A submit is counted with its answer

Before flume calls the merge queue to submit a place, it SHALL mark the
place as submitting and SHALL NOT count the submit. One write SHALL
count the submit, clear the mark and record the merge queue's answer.
The answer is the entry, the request, or the refusal with its words.
Thus no reader SHALL find a submit counted without its answer. Status
SHALL give `submitting` while the mark stands (48, 111, 124, core.14,
core.16).

#### Scenario: A reader between flume's writes

- **WHEN** a settle submits the place and the fake store reads it just before each write. The fake merge queue answers with a request, an entry or a refusal
- **THEN** a read finds the place submitting with no count. No read finds a submit counted with no request, entry or verdict. At the end, the place has one submit and no mark

#### Scenario: A refusal at submit

- **WHEN** the fake merge queue refuses the submit
- **THEN** the write that counts the submit gives the refusal as its verdict and the place waits on its caller with the merge queue's words
