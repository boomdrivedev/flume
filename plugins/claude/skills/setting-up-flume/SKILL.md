---
name: setting-up-flume
description: Set up flume in a repository. Write or change the `[profiles]` table of `.config/flume.toml`, which binds the place, change and merge-queue profiles, and read back whether flume can launch each kind there. Use when the user asks to set up, configure or adopt flume in a repository, to bind a merge queue or a change profile for flume, to explain the `[profiles]` table, or when a launch was refused because the repository binds no merge queue or no change profile. For the merge queue's own setup, it hands over to sluice's setting-up-sluice skill.
---

# Setting up flume in a repository

Do these steps in order. Show each command before you run it. Ask
before you commit, and before you change a file that the repository
already has.

## 1. Read how the repository stands

Run this in the repository, or give `--repository <folder>`:

```sh
flume status --binding --json
```

It reads and changes nothing: it fetches nothing and launches nothing.
flume host answers it, so the host's service manager must run
`flume host`. Read `status.repositories[0]`:

| field | what it says |
|---|---|
| `binding.read.branch`, `binding.read.revision`, `binding.read.from` | where flume read the binding: the default branch, as `origin` was last fetched (`origin`), or the repository's own branch (`local`) |
| `binding.read.declaration` | `.config/flume.toml` where that revision holds it, else `null` |
| `binding.read.binding` | the profile named for each seam: `place`, `change` and `merge_queue`, `null` where none is named |
| `merge_queue.profile`, `merge_queue.ready` | the merge queue named, and whether it lands the default branch |
| `merge_queue.words` | what is missing, where the merge queue is not ready |
| `kinds[].kind`, `kinds[].lands`, `kinds[].refused` | each kind flume can run there, whether it lands, and the words a launch of it is refused with by the binding |

When `binding` is `could_not_read`, its `reason` says why. Correct that
first.

## 2. Choose the profiles

flume binds no seam from another tool's files. The `[profiles]` table
alone binds each seam, so you must name each profile. Ask the user
which tools the repository uses:

| seam | profiles flume has | when it is needed |
|---|---|---|
| `place` | `git-worktree`, which is also the place profile when none is named | always; omit it to get plain git worktrees |
| `change` | `openspec` | a kind that reads a change, as flume's `change` does |
| `merge_queue` | `sluice` | a kind that lands, as each of flume's own kinds does |

## 3. Write the `[profiles]` table

Where `.config/flume.toml` does not exist, write it:

```toml
[profiles]
place = "git-worktree"
change = "openspec"
merge_queue = "sluice"
```

Where it exists, change only its `[profiles]` table. Keep its kinds and
every other table as they are. Leave out a seam that the user does not
bind.

flume reads the binding at the default branch. The table has no effect
until it is committed and the default branch holds it. Commit it the
way the repository takes work, and ask the user first.

## 4. Set up the merge queue

flume writes nothing of the merge queue's own setup. Where
`merge_queue.words` says that the merge queue does not land the default
branch, for example that `.config/sluice.toml` is missing, load sluice's
skill `sluice:setting-up-sluice` and follow it. It installs sluice,
writes `.config/sluice.toml` and gates the default branch.

## 5. Read back

Where the default branch lands on `origin`, run `git fetch origin`
first: the report reads `origin` as it was last fetched. Then run
`flume status --binding` again. The setup is complete when
`merge_queue.ready` is `true` and each kind the user will launch has no
`refused`. Tell the user each kind that is still refused, with its
words.
