## MODIFIED Requirements

### Requirement: The types of an agent, the terminal and a submission

The seams SHALL pass one type for each of: an agent's identity, its
start's secret and a stop-short that needs an answer or found work (29,
30, 33, 41, 92); an agent's command with the program's folder-trust
question, its answer and the program's kind (34, 88, C.4); the
terminal's clues and what its tidy closed and kept (138, 139); and a
submission with its outcomes, landed, failed, in conflict, held with the
forge's words where the forge refused its landing, or withdrawn (99, 111
to 113, 117, 125, 133). A held submission SHALL be neither decided nor
one a round moves, and a held outcome SHALL decide nothing (112). Each
SHALL serialize to JSON and back unchanged.

#### Scenario: An agent's command and a withdrawn submission round-trip

- **WHEN** an agent's command carrying its folder-trust question, its answer and its kind, and a submission withdrawn, are each written to JSON and read back
- **THEN** each equals the value written (34, 88, 125)

#### Scenario: A held submission and its outcome round-trip

- **WHEN** a submission held with the forge's words, a held outcome, a place's wait on a held landing and a held state whose submission's landing was refused are each written to JSON and read back, and a held state written before `landing_refused` existed is read
- **THEN** each equals the value written, the held submission is not open, and the earlier state reads with `landing_refused` false (99, 112, 133)
