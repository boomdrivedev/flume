## ADDED Requirements

### Requirement: The agent seam names an account's home's variable

The agent seam SHALL name the variable of an account's environment
whose value is its program's configuration home, the folder the program
keeps its settings in under that account, and none by default, so a
profile that names none gives its accounts no home (56, 171, C.4).

#### Scenario: A profile that keeps the seam's defaults

- **WHEN** an agent profile that keeps the seam's defaults is asked which variable names an account's configuration home
- **THEN** it names none (56, C.4)
