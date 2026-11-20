## ADDED Requirements

### Requirement: A profile's scenarios run side by side

The conformance suite SHALL offer a runner that takes a profile's
scenarios a bounded number at a time, each over a fresh harness made in
its own thread, and that names every scenario that failed in the
scenarios' order, as the one-at-a-time runner names them; a profile
whose harnesses share nothing, each on a server and in a scratch of its
own, SHALL run its scenarios on it (106).

#### Scenario: The herdr profile's terminal scenarios

- **WHEN** the herdr profile's terminal scenarios run in the tools layer
- **THEN** they run five at a time, each on a herdr server of its own, and a failure is named with its scenario's name in the scenarios' order (106)

### Requirement: A harness waits only for what a scenario awaits

The terminal harness SHALL give what a recipient's session took once it
took the number of inputs the scenario awaits, or by the harness's wait
where it takes fewer, and at once where none is awaited; and SHALL give
the seam a never-up program is opened on, its own by default or one
with the shortest bound on a start its terminal takes (43, 94, 95, 106).

#### Scenario: A session at work

- **WHEN** the scenario of a session at work reads what the session took before its send
- **THEN** it reads at once, and after the refused send it awaits one more input for the harness's wait and finds none (54, 95)

#### Scenario: A program that never comes up under herdr

- **WHEN** the herdr harness is asked for the seam a never-up program is opened on
- **THEN** it gives the profile with a start bound of 3,001 milliseconds, and the open answers not come up (35, 94)
