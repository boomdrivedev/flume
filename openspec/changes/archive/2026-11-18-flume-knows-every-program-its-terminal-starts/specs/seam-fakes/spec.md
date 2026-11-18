## ADDED Requirements

### Requirement: The fake agent scripts the models it takes

The fake agent SHALL take every model by default and SHALL script
taking none, refusing each name in the seam's default words; its
command SHALL pass `--model <name>` where the settings hand it one, its
log SHALL name each `model` it is asked, and its kind SHALL be `agent`
(88, 172).

#### Scenario: A model taken, then none

- **WHEN** a fix launches with `--agent model=large`, and then the fake agent is scripted to take no model and a second fix launches the same way
- **THEN** the fake's log shows `model` asked of `large` at the first's launch and start, its start's command passing `--model large`, and the second launch is refused in the seam's default words (88, 172)

### Requirement: The fake terminal scripts a kind it cannot start

The fake terminal SHALL start every kind by default, SHALL script a
kind it refuses, with words, until the script is lifted, and its log
SHALL name each kind it is asked of with the environment's variables by
name alone (C.5).

#### Scenario: A kind refused, then started

- **WHEN** a test scripts the fake terminal to refuse `agent` with words, asks it of `agent` and `other`, lifts the script and asks of `agent` again
- **THEN** the first is refused in those words, `other` and the last are started, and the log names each ask (C.5)
