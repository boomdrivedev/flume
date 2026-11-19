## MODIFIED Requirements

### Requirement: herdr runs the program the settings name

The herdr profile SHALL lead the pane's `PATH` with a folder of the
start's own linking the kind's name to the program the settings name,
or to flume's launcher where the command carries its words, SHALL give
the pane the host user's login environment under the agent's and a
shell of the start's own that reads no startup file, and SHALL close
and refuse, naming what ran, a start whose pane runs any other program
(29, 32, C.4, C.5; record 0018).

#### Scenario: Startup files that put another program first

- **WHEN** the host user's login shell is zsh, bash or sh, and its every startup file puts another `claude` ahead of the start's own folder on `PATH`
- **THEN** the pane's shell reads none of them, and the program the settings name starts

#### Scenario: A program that runs another in its place

- **WHEN** the program the settings name execs another `claude`
- **THEN** the start is closed and refused, naming the program that ran

### Requirement: The printed command

The printed-command profile SHALL answer open with the command, its
prompt carried in it, for a person to run in the place, the launcher's
words leading the program and its arguments where the command carries
them; refuse send and close; tidy nothing and show nothing; read the
agent stopped only once the agent's own record says its session ended
(134); give no screen (136); and settle nothing (32, 38, 122, C.5).

#### Scenario: A tell under the printed command

- **WHEN** a caller tells a place's agent under the printed command
- **THEN** the tell is refused with nothing typed (53)

#### Scenario: Seam scenarios over the printed command

- **WHEN** the terminal seam's scenarios run against the profile
- **THEN** each passes, its refusals among them

#### Scenario: The launcher's line

- **WHEN** the printed command opens a command of `amp` naming the launcher `/opt/flume/bin/flume` with `ack launched --`
- **THEN** its line runs `/opt/flume/bin/flume ack launched -- amp` in the place under the command's environment file (C.4, C.5)

## ADDED Requirements

### Requirement: herdr runs the launcher in the program's place

Where an agent's command carries the launcher's words, the herdr
profile SHALL link the kind's name in the start's own folder to the
launcher's program, and its agent start SHALL pass the launcher's
arguments, then the program's absolute path, then the command's
arguments; the pane's foreground SHALL then run the program the
settings name, as for any program (C.4, C.5).

#### Scenario: A program started through the launcher

- **WHEN** a command of the kind `amp` names as its launcher a scratch script that writes its process id and its arguments and then execs what follows them, under the pinned herdr
- **THEN** the kind's link in the start's folder leads to the script, the script is given the program's absolute path and the command's arguments, the program the settings name runs in the script's process, and the open is taken (C.4, C.5)

### Requirement: herdr reads the process its agent's program runs in

The herdr profile SHALL read, by `pane process-info` within its bound,
the process in the agent's pane's foreground running the program its
open checked, by its id; nothing where no foreground process runs it or
the pane is gone; unknown, in herdr's words, where herdr does not
answer, and where the agent's record, written before, names no program
(35, C.5; core.29).

#### Scenario: The launcher's process, then one started by hand

- **WHEN** an agent of the kind `amp` opens through a scratch launcher under the pinned herdr, and its program is then ended and run again by its path in the same pane
- **THEN** its process reads as the one the launcher wrote, and then as another, the one run by hand (35, S74)

#### Scenario: A pane that is gone

- **WHEN** the agent's pane is closed
- **THEN** its process reads as nothing running its program (35)
