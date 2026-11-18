## MODIFIED Requirements

### Requirement: The merge queue's submit holds no wait

The merge queue's submit SHALL answer once the merge queue holds the
submission, with no wait on its verdict, and its inbox SHALL give each
pending outcome of the place with the submission it decides, again at
every inbox until the place confirms it by the key it gives, and never
after; a merge queue that keeps nothing to confirm SHALL give no key and
each outcome once (117, 133).

#### Scenario: Submit holds no wait

- **WHEN** the fake merge queue holds a submission undecided
- **THEN** submit answers with the submission at once, and once it is decided an inbox gives its outcome with that submission, again until the place confirms it by its key, and then nothing (117, 133)
