## MODIFIED Requirements

### Requirement: Rex's send leaves nothing typed

The Rex profile's send SHALL clear held input with one `send-key
ctrl+c`, and only where the agent reads waiting. It SHALL then type the
text by `send`, in pieces of at most 60,000 bytes cut between
characters, and then press Enter by `send-key enter`. It SHALL never
interrupt a working agent. The profile SHALL read through the agent
whether the agent received the text (89, 95, C.5, core.11).

#### Scenario: A prompt left typed

- **WHEN** text is left typed in a waiting agent's block and a prompt is then sent
- **THEN** the agent takes the prompt alone, the text left typed cleared first (S43)

#### Scenario: A working agent sent a tell

- **WHEN** a tell is sent to an agent that reads working
- **THEN** no ctrl+c is pressed, and the agent takes the tell as its next input

#### Scenario: A text past sixty thousand bytes

- **WHEN** the profile sends a prompt of 150,000 bytes, non-ASCII among them, to an agent that reads its terminal three seconds after it reports idle
- **THEN** the agent takes the whole prompt in order as one prompt, within a minute on a busy host
