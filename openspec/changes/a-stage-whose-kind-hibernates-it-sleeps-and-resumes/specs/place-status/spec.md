## ADDED Requirements

### Requirement: Status gives a stage's hibernation and its resumes

Status SHALL give, of a place's current stage, whether its kind
hibernates it; its agent's `hibernate_after`, as written, with the
layer it came from, flume's own one hour named `default`; of one
hibernating, since when and why, `idle`, `caller` or `kept_open`; and
each resume of its current start, with when, what started it,
`caller`, `tell` or `answer`, its agent's identity, and whether it
carried its conversation or why not. The form SHALL gain these fields
and lose none (56, 61, 212, 213).

#### Scenario: A hibernating conversation

- **WHEN** status is asked of a place whose talk hibernated idle at t under the host's `[agents] hibernate_after = "30m"`
- **THEN** it gives talk hibernating since t, why `idle`, `hibernate_after` `30m` from `host`, and no live agent, in the JSON and the person's form (56, 212, S124)

#### Scenario: A resumed stage

- **WHEN** status is asked of a place whose talk was resumed by a tell and carried its conversation
- **THEN** it gives talk's one resume, by `tell`, its agent's identity ending `talk:1.1`, carried (56, 213)

#### Scenario: A resume not carried

- **WHEN** status is asked of a place whose talk's resume found its transcript gone
- **THEN** it gives the resume not carried with the profile's words why, naming no path, and talk's second start fresh (56, 213, S127; core.30)
