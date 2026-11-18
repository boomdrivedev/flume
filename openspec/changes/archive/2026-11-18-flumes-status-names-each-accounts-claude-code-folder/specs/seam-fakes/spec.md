## ADDED Requirements

### Requirement: The fake agent scripts its home's variable

The fake agent SHALL name no variable as an account's configuration
home until a test scripts one, and then SHALL name the variable
scripted, so the engine's scenarios read a home through the seam as a
profile gives it (56, 106, C.4).

#### Scenario: A scripted home's variable

- **WHEN** a test asks the fake agent which variable names an account's home, scripts it to name `FAKE_HOME`, and asks again
- **THEN** the first asking names none and the second names `FAKE_HOME` (56, 106)
