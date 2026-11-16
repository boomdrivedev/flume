## MODIFIED Requirements

### Requirement: Each open change is validated as it would archive

`root:changes` SHALL run `openspec validate --changes --strict`, then
archive each open change alone, by `openspec archive`, into a scratch
copy of the tree's specs and configuration, every spec that archive
writes passing `openspec validate --strict`. It SHALL fail naming each
change that fails, with OpenSpec's words, fail none for a task left
unchecked, and write nothing in the tree (core.59).

#### Scenario: A change archive refuses

- **WHEN** `root:changes` runs over an open change that adds a requirement its spec already holds
- **THEN** it fails naming the change and saying `openspec archive` refuses it, with OpenSpec's words naming the requirement

#### Scenario: A new capability with no Purpose

- **WHEN** `root:changes` runs over an open change that adds a capability whose delta gives no `## Purpose`
- **THEN** it fails naming the change and the spec its archive writes, with OpenSpec's warning that the Purpose is a placeholder

#### Scenario: A modified requirement past OpenSpec's length

- **WHEN** `root:changes` runs over an open change that modifies a requirement to more than 500 characters of text
- **THEN** it fails naming the change and the spec its archive writes, with OpenSpec's warning that the requirement's text is very long

#### Scenario: Each change alone, its tasks open

- **WHEN** `root:changes` runs over two open changes that each add a requirement of one name to one spec, each with a task left unchecked
- **THEN** it passes, and the tree's specs and changes are as they were before it ran
