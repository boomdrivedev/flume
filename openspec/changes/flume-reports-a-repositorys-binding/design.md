# Design: flume reports a repository's binding

## Context

A caller that offers a launch must know before the launch whether flume
can run the kind in the repository. In a launch, flume reads the binding
at the default branch after one fetch. Then it refuses a kind that lands
where the binding names no merge queue (launch.rs). A caller cannot read
that itself, because the binding and whether a kind lands are flume's
facts. The runner reads flume only through `flume host`, by one
subscription to status (Boom invariants `one-subscription-per-plugin`
and `clients-never-fetch-or-start-a-tool`).

## Decisions

### 1. A part of status, not a command of its own

The report is a part of status: the ask names the repositories, and the
answer gives `status.repositories`. The runner holds one subscription to
status, and each document of that subscription holds the report. A
command of its own would need a second connection to `flume host`, or a
second ask on a timer. flume finds no clone by its address (core.55), so
the caller names each repository by its folder. A status that names no
repository gives no `repositories` field, so the answer of each earlier
caller stays the same (core.14).

A `line` ask names no repository. `flume host` refuses an ask that names
both, and the command line refuses `--binding` with `--line`.

### 2. It fetches nothing

Status runs, sends and writes nothing (Boom invariant
`status-reads-each-fact-where-it-lives-and-changes-nothing`). A fetch
writes refs, and a fetch for each read of each subscription breaks Boom
invariant `one-forge-read-per-repository-per-interval`. Thus the kinds
seam reads the binding at `origin/<default branch>` as the last fetch
left it, else at the repository's own branch. The report says which in
`from`. A launch still reads the binding after its own fetch (Boom
invariant
`a-repositorys-kinds-are-read-at-its-bases-revision-and-its-binding-at-its-default-branch`).
Where origin moved after the last fetch, the report can be older than a
launch's read. The next fetch of a settle or a launch corrects it.

### 3. Whether the merge queue lands the default branch

The merge-queue seam gains lands-on: whether the merge queue lands a
base in a repository. The profile reads it from what the repository
holds, with no fetch and nothing run. The sluice profile reads
`.config/sluice.toml` at the base and at each remote's copy of it, as
its base read does. It says `Not` with the file that is missing, or with
the base that its
`[branches]` does not name. The report asks it for the default branch,
the base of a launch that names no base.

### 4. The kinds

The report gives flume's own kinds, and the kinds that the repository
declares at the revision of its binding. It also gives each kind that
the repository's lock records from a source of the host, resolved as a
launch resolves it. For each kind, it gives `lands`, and `refused`: the
words of the binding's refusal. A kind that lands is refused where the
merge queue is not ready. A kind that reads a change is refused where no
change profile is bound. `lands` is `null` where flume could not read the
kind, and `refused` then says why.

### 5. The JSON form

`status.repositories` is a list. Each item is:

```json
{
  "repository": "/abs/path/to/clone",
  "binding": {
    "read": {
      "branch": "main",
      "revision": "4f1c2a9",
      "from": "origin",
      "declaration": ".config/flume.toml",
      "binding": { "place": null, "change": "openspec", "merge_queue": "sluice" }
    }
  },
  "merge_queue": { "profile": "sluice", "ready": false,
                   "words": "sluice does not land main: no .config/sluice.toml at main or a remote's main" },
  "kinds": [
    { "kind": "flume/fix@1", "lands": true,
      "refused": "the kind flume/fix@1 lands, and no merge queue lands it: sluice does not land main: no .config/sluice.toml at main or a remote's main" }
  ]
}
```

`from` is `origin` or `local`. `declaration` is `null` where the revision
holds no `.config/flume.toml`. `binding` is
`{"could_not_read": {"reason": "..."}}` where the binding cannot be read.
`words` and `refused` are absent where there is nothing to say.

### 6. The setup skill

The Claude Code plugin gains `skills/setting-up-flume/SKILL.md`. It
reads the report, writes `[profiles]`, and hands the merge queue's own
setup to sluice's skill `setting-up-sluice`, as each tool owns its own
setup. The binary writes the skill into the plugin that it gives its
agents, as it writes each file of the plugin.

## Risks

- The report can be older than origin until the next fetch (decision
  2). The launch reads origin again and refuses with the same words.
