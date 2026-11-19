## Purpose

A statechart kind's command stage: a stage that runs one command its
kind declares in place of an agent, filled from the launch's
parameters, run through the terminal by flume's launcher, ended by its
recorded exit and the place's branch, its failure taken on as the kind
says, and run again by a retry (223, 224, S130).

## ADDED Requirements

### Requirement: flume binds the command template

flume SHALL bind a second template, `command`, beside `stage`: a state
whose `machine` is `command` is a command stage, which asks for its one
session, its command's run, and ends `passed` once the run delivered or
`failed` once it failed, the kind's transitions reading each as they
read the stage template's ends; a place SHALL hold both templates' text
as its launch read them (175, 223, C.2).

#### Scenario: A command stage delivers

- **WHEN** a repository declares update@1, whose first stage `move` runs `bin/move {{params}}`, and it is launched with parameters on a host whose bound is spent
- **THEN** the move's command runs at once through the terminal as a program and no agent opens, its `{{params}}` the path of the place's copy of the parameters, and once its launcher records it exited 0 with a commit on the place's branch the stage is delivered and the place is submitted (223, 224, S130)

### Requirement: A launch carries parameters

A launch SHALL carry parameters beside its brief as its caller wrote
them; flume SHALL keep them with the place, write them whole to the
place's own folder as `params`, which a command's `{{params}}` names,
and read, rewrite and run no word of them. A launch that gives none
while a command stage's command asks for them SHALL be refused naming
the stage, making nothing (4, 224, C.7).

#### Scenario: No parameters for a command that asks for them

- **WHEN** update@1's move asks for `{{params}}` and the launch gives none
- **THEN** the launch is refused naming `main.move` and the parameters, and nothing is made (4, 224)

### Requirement: A kind's command stage flume cannot run is refused

A launch SHALL be refused, naming the stage and making nothing, where a
command stage names no command, names a program by an absolute path or
a path through `..`, names a prompt or agents, delivers anything but its
commits, or names for its `on_fail` a state that runs no stage
template; and where any stage sends a place back to a command stage (4,
157, 158, 223).

#### Scenario: A send-back to a command stage and a program outside the repository

- **WHEN** update@1's `fix` names `move` for its `on_fail`, or its move runs `/usr/local/bin/move` or `../bin/move`
- **THEN** each launch is refused naming `main.move`, and nothing is made (4, 223)

### Requirement: A command stage runs as a program

A command stage's session SHALL start once its kind reaches it,
whatever the host's and its stream's bounds, taking no room, no account
and no points, with no prompt and no agent's settings, under a fresh
identity and secret, its start's record marked a command's; the
stream's terminal SHALL run it as a program of a pane or block, by a
link of the start's own named for the place's kind that leads to
flume's launcher, never as an agent of the terminal (70, 145, 223, C.5).

#### Scenario: The run's command and record

- **WHEN** update@1 is launched with parameters
- **THEN** the terminal logs one run and no open, the command carries flume's launcher and the kind's name for its link, and the start's record is marked a command's with no account (223, C.5)

### Requirement: flume's launcher runs a command as its child

flume's launcher SHALL check a command start as it checks an agent's,
run the command as its child in the place, keep its output whole in
`output.log` and its exit beside the start's record, and then start a
settle of the place (223, C.2).

#### Scenario: A command run by the launcher

- **WHEN** the launcher of a command start runs a program that prints and exits 3
- **THEN** its output is kept whole in `output.log`, its exit 3 is written beside the start's record, and the launcher exits 3 (223, C.2)

### Requirement: A command stage's end is read from its recorded exit

A command stage SHALL deliver once its launcher recorded that its
command exited with success and the place's branch holds a commit made
since it started, and SHALL fail where its command exited otherwise,
exited with nothing committed, began otherwise than its start
recorded, or stopped with no exit recorded (25, 223).

#### Scenario: A command commits nothing

- **WHEN** update@1's move exits 0 with nothing committed
- **THEN** the move's session is failed (25, 223)

### Requirement: A command stage's failure is an end status gives

A command stage's failure SHALL be an end of the place, its words
saying how the command exited and where its output lies, and status
SHALL give the stage's state and, once it ended, its exit and the
output's path (43, 56, 223).

#### Scenario: A command exits with an error

- **WHEN** update@1's move exits 2 with nothing committed
- **THEN** the move's session is failed, status gives its exit 2 and its `output.log`'s path, and its words say it exited with 2 (56, 223)

### Requirement: A failure is taken on as the kind says

A command stage's failure SHALL end its stage `failed` while its
failures taken on are under its bound, its `max_send_backs` or flume's
own; the kind's transition takes it on, and a stage that runs an agent
reached from it SHALL start with the brief and the failure as its
words. Past the bound the stage keeps working and the place waits on
its caller (23, 112, 223).

#### Scenario: The fix starts with the failure

- **WHEN** update@1's move exits 2 and its `failed` is taken to `fix`
- **THEN** the fix stage's agent starts with a prompt holding the brief and the move's failure (112, 223, S130)

### Requirement: A retry runs a command again

A retry naming a command stage, by its path or its session, SHALL run
its command again from the place's head under its next start, and one
with words SHALL be refused naming the stage (49, 223).

#### Scenario: A retry with and without words

- **WHEN** update@1's move failed past its bound, and its caller retries `main.move` with words, then with none
- **THEN** the first is refused naming `main.move#1/1` as a command stage, and the second runs the command again under start 2 (49, 223)

### Requirement: A caller's stop ends a command

A caller's stop SHALL end a command stage's run whether or not it
works, since it is no agent, and leave its stage to a retry (51, 223).

#### Scenario: A stop and a retry

- **WHEN** update@1's move runs, its launcher's word standing, and its caller stops the place with no force
- **THEN** the stop is taken, the terminal closes the run, the move's session reads stopped, and a retry naming `main.move` runs the command again (49, 51, 223)
