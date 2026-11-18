## ADDED Requirements

### Requirement: herdr starts a program it reads through its integration only once it is installed

herdr's profile SHALL start `omp` and `mastracode`, whose state the
pinned herdr reads only through its integration, only while `herdr
integration status`, run under the login environment with the given
one over it, reports the kind's integration in a state other than `not
installed`, which SHALL be refused naming `herdr integration install
<kind>` (4, C.5).

#### Scenario: omp's integration not installed, then installed

- **WHEN** herdr's profile is asked whether it starts `omp` under a scratch home where the pinned herdr reports omp's integration not installed, and again after `herdr integration install omp` there with the home's `.omp/agent/extensions` made
- **THEN** the first is refused naming `herdr integration install omp`, and the second starts it (4, S75)

### Requirement: herdr's integration status read within its bound

herdr's profile SHALL run `herdr integration status` within its bound,
and SHALL refuse a status that does not answer within it, or names no
line for the kind, naming what herdr gave; it SHALL start every kind
herdr reads on its own with no call to herdr (C.5; core.29).

#### Scenario: herdr's status lines read

- **WHEN** the profile reads `omp: not installed (<path>)`, `omp: current (v10) (<path>)`, `omp: outdated (v8 < v10) (<path>)`, and a status naming only `mastracode`, each for `omp`
- **THEN** the first is refused naming the install, the second and third start it, and the fourth is refused naming what herdr gave (C.5)

#### Scenario: A kind herdr reads on its own

- **WHEN** herdr's profile is asked whether it starts `amp` and `claude` with no `herdr` it can run
- **THEN** it starts each (C.5)
