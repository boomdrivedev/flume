## ADDED Requirements

### Requirement: The store reads a file of a place's folder

The store seam SHALL read a file under a place's own folder by its
absolute path, as written: none where there is none, and why not where
it cannot be read or lies outside the folder (223, C.7).

#### Scenario: A file read back

- **WHEN** a file is written under a place's folder and read back, and a path outside it is read
- **THEN** the first reads its text whole and the second is refused naming the path (223)

### Requirement: The terminal seam runs a command

The terminal seam SHALL give `run`: a command stage's command run under
an identity as a program of a pane or block, through flume's launcher,
never an agent of the terminal; a terminal that runs no program but an
agent refuses it in its words. The host SHALL give the words that run
flume's launcher, the binary by its absolute path (223, C.4, C.5;
core.60).

#### Scenario: A terminal that runs no command

- **WHEN** a terminal profile that keeps the seam's default is asked to run a command
- **THEN** it refuses, naming the program (223, C.5)
