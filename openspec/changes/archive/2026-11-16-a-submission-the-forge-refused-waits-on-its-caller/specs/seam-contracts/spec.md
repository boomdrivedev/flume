## MODIFIED Requirements

### Requirement: The types of an agent, the terminal and a submission

The seams SHALL pass one type for each of: an agent's identity, its
start's secret and a stop-short that needs an answer or found work (29,
30, 33, 41, 92); an agent's command with the program's folder-trust
question, its answer and the program's kind (34, 88, C.4); the
terminal's clues and what its tidy closed and kept (138, 139); and a
submission with its outcomes, landed, failed, in conflict, held or
withdrawn (99, 111 to 113, 117, 125, 133). Each SHALL serialize to JSON
and back unchanged.

#### Scenario: An agent's command and a withdrawn submission round-trip

- **WHEN** an agent's command carrying its folder-trust question, its answer and its kind, and a submission withdrawn, are each written to JSON and read back
- **THEN** each equals the value written (34, 88, 125)

#### Scenario: A held submission and its outcome round-trip

- **WHEN** a submission held with the forge's words, a held outcome, a place's wait on a held landing and a held state whose submission's landing was refused are each written to JSON and read back, and a held state written before `landing_refused` existed is read
- **THEN** each equals the value written, the held submission is not open, and the earlier state reads with `landing_refused` false (99, 112, 133)

## ADDED Requirements

### Requirement: A held submission is neither decided nor moved

A submission held where the forge refused its landing SHALL carry the
forge's words, or words saying the merge queue's record gives none, and
SHALL be neither decided nor one a round of its base's queue moves,
staying the place's open submission until it lands, goes back to the
queue, is withdrawn or is replaced; a held outcome SHALL decide nothing
(99, 112, 124, 125, 133; core.58).

#### Scenario: A round moves no held landing

- **WHEN** a submission's state is held with the forge's words
- **THEN** it is not open, so no round of its base's queue moves it, and it is none of the decided states, landed, failed, in conflict or withdrawn (112, 125)
