## MODIFIED Requirements

### Requirement: The root project checks what spans the tree and reads no Markdown

The root project SHALL hold one check-fast task per check that spans
the tree: `format`, `toolchain`, `specs` and `changes` (`openspec
validate --strict` of the specs and the open changes) and `marketplace`
(the marketplace manifest and each plugin it lists); no task SHALL read
a Markdown file for its words or its form, a record's number among them
(core.21; record 0074).

#### Scenario: An open change that does not validate

- **WHEN** `root:changes` runs over an open change whose requirement has no scenario
- **THEN** it fails naming the change and the requirement
