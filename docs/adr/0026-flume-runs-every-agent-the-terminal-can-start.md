---
number: 26
title: Flume runs every agent the terminal can start, each part of the agent seam built for its program or plainly degraded
status: accepted
date: 2026-10-30
decision-makers:
- Chuck Swanberg
---

# Flume runs every agent the terminal can start, each part of the agent seam built for its program or plainly degraded

Status: accepted by the user on 2026-10-30, on flume-sluice-build's card (req_bb65e1d78dd14c8d859f, "Approved", no note).

Deciders: the user, 2026-10-30: "Are the mode values properly encapsulated along the agent seam so that it won't be awkward when we go to add support for other agents? In fact we should be supporting all agents out of the box since our multiplexer supports all the agents". What the agent seam requires of a program, what flume does where a program lacks a part, how a profile is declared, where the line is drawn, how a model is named, which program a stage with none named runs, and which profiles are built are the reviewing agent's.
Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at a11f6d5, clauses 3, 31, 32, 35, 38, 88, 89, 93, 105, 135, 144, 164, 170 to 174, B.4, C.4, C.5 and S73 to S75, as amended with this record; records 0003, 0016, 0017, 0018, 0020, 0021, 0024; rules core.1, core.2, core.9, core.10, core.29, core.30; `crates/flume-seams/src/agent.rs` (the `Agent` trait: `command`, `started`, `received`, `asking`, `own_work`, `ended`, `last_words`, `spent`, `started_without`, `stop_short`, `send_back`, `guard`, `on_quiet`), `crates/flume/src/profiles.rs` (`agent`, which binds Claude Code's profile and resolves any other program to nothing), `crates/flume/src/profiles/terminal/herdr/mod.rs` (`KINDS`, herdr 0.9.5's 24 agent kinds); herdr 0.9.5's `agent start --help` (`--kind`, "Success means the expected agent was detected in the same terminal and is ready for input"), `agent prompt --help` (`--wait`, `--until`, `agent_prompt_stalled`), `agent explain --help` and `integration status`; herdr's documentation at <https://herdr.dev/llms.txt>, its Agents page (which agents herdr reads by screen, which report their own state, "state requires the integration" for OMP and MastraCode, "state only, less tested" for Gemini CLI and Cline, idle with background tasks still running for Antigravity and Grok, and "Adding a completely new agent still requires a Herdr binary update"); the `--help` of Codex 0.165.0, Gemini CLI 0.66.0, GitHub Copilot CLI 1.0.104 and OpenCode 1.19.4, read on 2026-10-30.

## Context and problem statement

flume has one agent profile, Claude Code's (C.4), and `profiles.rs` resolves any other program to nothing, so a launch that names one is refused. Its terminal, herdr, starts and reads 24 kinds of agent program: pi, claude, codex, gemini, cursor, devin, agy, cline, omp, mastracode, opencode, copilot, kimi, kiro, droid, amp, grok, hermes, kilo, qodercli, qwen, letta, maki and muse. For each, `herdr agent start --kind <kind>` starts the program in a pane and answers once it is ready for input, `herdr agent prompt` submits a prompt and can wait for the turn it starts, and herdr reads the agent's state, working, idle, done or blocked, from its screen or from the reports of its own integration. A stream's pool may hold accounts of several programs (record 0020), and a mode is now flume's own word that each profile maps (record 0024). The user asked that every agent herdr runs be supported out of the box.

The seam asks more than herdr gives. Claude Code's profile answers every part from the agent's own record through flume's plugin: its word that it started under its start's identity and settings, that it received a prompt, that it asks a person, its background work, its usage limit, its last words, a guard on submitting, a send-back while its deliverable is missing, and a settle the moment it goes quiet. Most programs have hooks of some kind, but each in its own form and release, and some have none. Clause 93 says an answer the agent's own record does not give is unknown, never guessed from the terminal, and clause 31 that an agent whose own work flume cannot read is never quiet: as written, a program with no profile of its own could start but its stage could never end.

## Considered options

For which programs flume runs:

1. Only programs with a profile built for them, as today; each new program waits for its profile.
2. Every program herdr has a kind for: a profile built for a program where flume carries one, and one common profile for every other program, reading what every program gives; each part of the seam either built or degraded, as the profile declares.
3. Every program, the common profile reading the terminal for every part, and no profile of a program's own.

For how a program's entry is declared, its name, its terminal kind, its model flag and its map of modes:

1. A table built into flume, one entry per program, each checked against the release flume pins.
2. Data shipped with flume, a file per program that flume reads at its start.
3. Both, a host's data adding programs or overriding the built-in table.

For an agent's start where the profile reads no word of the program's own:

1. herdr's ready alone.
2. A launcher of flume's own, run in the pane under the start's identity and secret, which checks the start's record and then becomes the program, its word with herdr's ready the start.

For an agent's model:

1. The program's own name, passed as written, a layer giving one name or a table of programs as for a mode.
2. Flume's own names for classes of model, mapped by each profile.

For the program of a stage whose layers name none:

1. flume's default program, `claude`, always.
2. The program of the account chosen from its pool, among the accounts whose program can run the stage; flume's default only where there is no pool.

For the profiles built first:

1. Claude Code's alone, the common profile later.
2. The common profile, with entries for the programs the host can check today, then Codex's own profile.
3. Full profiles for Codex, Gemini CLI, Copilot CLI and OpenCode before the common profile.

## Decision outcome

Programs option 2, declaration option 1, start option 2, model option 1, program option 2, and profiles option 2.

**Every program herdr starts is a program flume runs** (170). Each is bound by an agent profile: its own, where flume carries one, and otherwise the common profile. Out of the box means that every one of herdr's 24 kinds is accepted as a program; that a stage whose settings give no model and no mode starts any of them with nothing more; and that each part of the agent seam is either built for the program or degraded as its profile declares, and status names each degraded part with the agent, so nobody reads a degraded part as a built one. A profile of a program's own replaces the common profile's readings part by part, so a program gains its parts one at a time, each checked against its pinned release.

**The common profile reads what every program gives** (171). Where the profile reads no word of the program's own for a part:

- **Start.** The program is the herdr kind's canonical executable, or a path the settings name (C.5's check of the pane's foreground holds for it as for Claude Code). The start runs flume's launcher in the pane, under the start's identity and secret in its environment: it compares the digest of each variable the start's record names with its own environment, keyed by the secret, and the program it is to run with the one the settings name (161), records the start's word with its own process id and whether anything differed, and then becomes the program in the same process. The agent is started once that word is recorded with nothing differing and the terminal reads that process, by its id, running the program and ready for input; a program in the pane under another process is not that start's agent, as a resumed or restarted one would be. So the start check's program and environment are generic, and the mode is compared only where a profile reads it back (record 0024).
- **Prompt delivery and ready for input.** herdr's `agent start` answers once the program is ready for input, and `agent prompt` submits the prompt, for every program; a program herdr reads blocked at its start is held at a question as C.5 says. Where the profile has no word of the program's own that it received a prompt, `agent prompt --wait --until working --until blocked` within the 60 seconds of 38 is read as received: herdr's word that the turn the submission began was seen; `agent_prompt_stalled` or a timeout is read as not received, and the prompt is sent once more as 38 says.
- **Working, quiet and waiting.** The terminal's state, as it is for every program today (96): working as working, idle and done as waiting. Blocked is asking where the profile leaves no mark of its own (90); Claude Code's profile keeps its mark.
- **Own work** (31, 91). Where the profile reads none, the agent's own work is read as none once the terminal has read it waiting at two settles a minute or more apart. Clause 31 would otherwise keep such an agent from ever being quiet and its stage from ever ending. Background work an agent started and that the terminal cannot see, as herdr says of Antigravity and Grok, can be cut short; status says own work is read from the terminal, so the kind's author and the host know.
- **Usage limits** (146, 151). Unknown, the seam's default today. An account whose limit the profile cannot read is never recorded spent; its agent goes quiet without its deliverable and its stage is stuck at its limit (44) with the screen as its last words, for its caller to retry on another account. Status says spent is not read.
- **Last words and ended** (43, 134). The terminal's screen, and its stopped.
- **Hooks.** Optional. With no hook flume can call, send-back (115), guard (116) and on-quiet (121) are not offered, as the seam already allows: the agent may go quiet with its deliverable missing and is read stuck at its limit; a submission that gets past no guard is read as any other of the place's branch (108), and status names the guard not offered; and herdr's watch settles the place at each change of the agent's state (122).
- **Stop-short.** `flume needs` for every program, run by the agent from a shell command with its start's identity and secret in its environment (41, 92). A program that keeps the environment from the commands it runs cannot stop short, and status says so.
- **Model.** The program's own name for a model, passed as written (172), by the flag its entry names; flume maps no model's name from one program to another. A layer may give a table of programs, `model = { claude = "opus", codex = "gpt-5.5" }`, as it may a mode. A program whose entry names no model flag refuses a model given, naming the program.

**A built-in table, one entry per program.** Each entry gives the program's name, its herdr kind, its model flag where it has one, its map of the four modes (record 0024), a flag that trusts the folder where it has one, and which parts a profile of its own builds. The table is code, checked as every profile is against the release flume pins, so an entry passes the conformance suite or is no entry (106; core.29). Every herdr kind has an entry from the first build, holding the program's name and kind; an entry with no model flag refuses a model, and one with no map of modes refuses a mode, so each is accepted and plainly degraded until its flags are checked. Part C writes each entry's flags when they are built.

**Where the line is drawn.**

- A program herdr has no kind for does not run under herdr (C.5); under the printed command, any program with an entry runs.
- A program with no model flag runs with the model its account's configuration chooses, and a model given refuses.
- A program whose mode map lacks a mode refuses that mode (169), never runs wider or narrower.
- A stage that delivers a plan runs only on a program whose profile offers the plan part (164), Claude Code's today.
- A program whose start asks a question flume does not answer is never begun with the screen (34, 35); the common profile answers the folder-trust question only where its entry names a flag or a question checked against the release.
- A program whose herdr state needs herdr's integration, OMP and MastraCode in herdr 0.9.5, refuses the launch while `herdr integration status` reports the integration not installed, naming the command that installs it.
- A program herdr reads by its screen alone, Gemini CLI and Cline among them, runs, and status names its state as read from the screen.

**The program a stage with none named runs** (173). Where no layer names a program and the agent has a pool, the account chosen names it: the pool's usable accounts for the stage are those whose program can run it, its mode mapped, its model given or not needed, and, for a plan stage, the plan part offered (164, 144). So a kind that names no program and gives flume's modes runs on whichever program the stream's pool offers, as record 0020 meant a pool of several programs to. Where there is no pool, flume's default program, `claude`, runs it, as today.

**Why not the others.** Programs option 1 keeps every program but Claude Code out until its whole profile is built, which is what the user asked to change. Option 3 would degrade Claude Code's profile to the terminal's readings and throw away the guard, the send-back, the usage limit and the received ack it already has. Declaration option 2 lets a file flume did not test map a mode: an entry that put `edits` on a program's full-access flag would run every agent of it wider than asked, and nothing would check it against the release; herdr itself adds an agent only with a binary update. Option 3 adds a second source for the same entry, and a host's override is that unchecked file again. Start option 1 would take herdr's ready, which the herdr profile already reads as a second signal and never the start (35, C.5), since herdr reads Claude Code's first-run screens as ready; with no word under the start's identity and secret, a pane brought back without its environment, or a program a person started by hand, would be taken as the stage's agent. Model option 2 would need flume to know every program's models and keep up with them; the program's own name, scoped by a table where a host mixes programs, is what a host writes today. Program option 1 would leave a pool's accounts of every program but Claude Code unusable for any stage whose layers name no program, which is every flume kind.

### Consequences

- Clauses 3, 31, 32, 35, 38, 88, 89, 93, 105, 135, 144, 164, C.4 and C.5 of flume's statement are amended, and 170 to 174 and S73 to S75 added, at blueprints a11f6d5. Rule core.1's details, core.10 and core.30 are amended and core.39 drawn from this record.
- The agent seam gains a profile's declaration of which parts it builds, which status reads to name the degraded ones; the common profile and the launcher are flume's own, in `profiles/agent/`, and name no program outside its table (core.1).
- The herdr profile's `send` waits on `agent prompt` for the turn where the agent's profile reads no received ack, and its `open` accepts every kind in `KINDS`, as it already does.
- A host with a pool of several programs gains nothing until it writes accounts of them (144); a host that writes none sees no change.
- Public docs say "every agent program herdr starts", and name the programs as products.

## Amendment, 2026-11-18: how a profile says which parts it builds

Status: accepted, the direction unchanged. The choices below are the construct stage's of the change `each-agent-profile-says-which-parts-it-builds`, made on the rules written and cheap to change, each written with the path not taken.

Deciders: the change's construct stage.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clauses 93, 170, 171 and 174, C.4's parts row and S73; `crates/flume-seams/src/agent.rs` (`send_back`, `on_quiet`, `reports_windows`, `mode_read_back`, and the defaults of `spent`, `windows`, `carry` and `own`); `crates/flume-core/src/status.rs` and `accounts.rs`; `crates/flume-conformance/src/clauses.rs` (`PROFILES`); the change's design, `openspec/changes/each-agent-profile-says-which-parts-it-builds/design.md`.

### Decision outcome

- **The parts a profile may degrade are the eight flume reads a lesser way today**: spent, windows, send-back, guard, on-quiet, the mode, carry and own settings, in 171's order. Command, started and stop-short are never degraded (C.4). Received, asking, own work, ended and last words join with the common profile's change, which has flume read them from the terminal (171); usage and plan join with the changes that add them to the seam (152, 164). Not taken: every part 171 lists at once, which would have status say flume reads a part from the terminal before it does.
- **A profile names the parts it builds, and how a degraded part is read is flume's**, one sentence per part on the seam's type, from 171. Not taken: each profile's own words for a degraded part, which 93 and 171 leave to no profile.
- **A profile that declares nothing builds none**, so no degraded part is read as a built one (170); Claude Code's profile names each of its eight in turn, so a part added later is built for it only by the change that builds it. Not taken: a method every profile must write, or every part built by default.
- **Whether send-back and on-quiet are offered, windows reported and the mode read back is read from the parts**, no profile answering them itself; the answers that read a record, `spent`, `windows`, `guard`, `carry`, `own` and `reserved`, answer the lesser way for a part not built, which the agent seam's scenarios hold every profile to. Not taken: each profile's own four answers beside its parts.
- **Status gives the degraded parts alone**, as the agent's `degraded`, each with its part and how it is read, left out where none is (core.14). Not taken: every part, built or degraded.
- **The clause list builds 174**; 170 to 173 and S71 to S75 stay the common profile's. The statement holds as written: C.4's Claude Code rows each answer from its record, so its table takes no parts row. Not taken: that row on a blueprints branch.

## Amendment, 2026-11-18: how the launcher runs and how its start is read

Status: accepted, the direction unchanged. The choices below are the construct stage's of the change `a-program-without-its-own-profile-starts-through-flumes-launcher`, made on the rules written and cheap to change, each written with the path not taken.

Deciders: the change's construct stage.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clauses 35, 161, 171 item 1, C.4's common-profile command and started rows, C.5's herdr row and S74; `crates/flume/src/profiles/agent/claude/hook.rs` (`compare`, `began_otherwise`, `settle_detached`), `crates/flume/src/profiles/agent/common.rs`, `crates/flume/src/profiles/terminal/herdr/mod.rs` (`open`, `runs`), `crates/flume-core/src/reading.rs`, `deliver.rs` and `unsettled.rs`; the terminal-profiles spec's "Rex runs only a program that reports its state to it"; the change's design, `openspec/changes/a-program-without-its-own-profile-starts-through-flumes-launcher/design.md`.

### Decision outcome

- **The launcher is `flume ack launched -- <program> [<argument>…]`**, a subcommand of flume's one binary beside `ack started` and `ack received`, in `profiles/agent/launcher.rs`. Not taken: a launcher binary of its own, a second artifact to build and ship with nothing flume's binary lacks; and flume acting as the program when a link named for the kind runs it, which hides the subcommand from its help and from the pane's command line.
- **It compares as Claude Code's started word does**, each variable's digest keyed by the secret, by one comparison both share, and the program it is to run with the record's, a name found on `PATH` past any file that is the launcher's own binary, since the kind's name in the start's folder leads to the launcher. Not taken: the launcher finding the program itself from the record, which would leave nothing to compare.
- **Its word is written once**, by create-new, at `launched/<identity>.json` in the place's own folder, naming its process; a second launcher under the start, as a person typing the kind's name in the pane runs, runs nothing. Not taken: a word at every run, which would let a second launcher name a new process as the start's.
- **It becomes the program by exec, in its own process.** Not taken: the program as its child, which would put it in another process than the word names.
- **The seams gain only defaults**: the agent seam's `launches` and `launched`, the command's `launcher` words, which a terminal takes whole, the terminal seam's `program_process`, and `Unsettled::process`. Not taken: a richer `started` answer, which every profile and fake would change; and the launcher a part a profile builds, which 174 would let a profile degrade where started is never degraded.
- **The core reads a launched start in one place**, for a settle, a tell and the stop check alike: started only with the word, the terminal reading the program in the word's process and the agent waiting; begun otherwise where the terminal reads the program in another process, so 161's end and fresh start follow. Not taken: the agent profile asking the terminal, which crosses the seams (core.2); and a check made once at the open, which misses a program run by hand after it (S74).
- **herdr and the printed command run the launcher; Rex runs none**, since it opens no program but `claude`. Under the printed command, which reads no process, such a start reads unknown; the prompt's argument there (C.4) is left to the change that builds it.

## Amendment, 2026-11-18: how the core holds a launched start once taken

Status: accepted, the direction unchanged. The choices below are the code stage's of the change `a-program-without-its-own-profile-starts-through-flumes-launcher`, made on the rules written and cheap to change, each written with the path not taken.

Deciders: the change's code stage.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clauses 35, 161 and S74; `crates/flume-core/src/reading.rs` (`Engine::started`, `Engine::began_otherwise`, `Engine::began_otherwise_held`), `end.rs` (`read_end`), `deliver.rs`; `crates/flume/src/profiles/agent/launcher.rs` (`launch`); `crates/flume/src/profiles/terminal/herdr/mod.rs` (`program_process`).

### Decision outcome

- **The agent waiting is read only for a start not yet taken**, one sent no prompt and not begun; a start taken reads started while the terminal reads its program in the word's process, working or not. Not taken: the agent waiting at every reading, which would read a begun agent at work no agent of its start's and end its stage lost.
- **A begun start is read begun otherwise too**: the reading of a begun stage's end first reads whether its program runs in another process than its launcher became, so a program run again by hand after its prompt is ended once quiet and its stage started afresh, as S74 has it, never read lost; and a reading of another process, once the place holds it, stands for that start at the settles that follow, though the terminal then reads nothing running the program. Not taken: the terminal's reading alone at each settle, which a program between its two processes would turn into a lost stage.
- **A program the launcher cannot find runs nothing and writes no word**, the launcher saying which. Not taken: a word saying the program differed, which a program found nowhere does not show.
- **herdr reads a pane gone at its process reading as nothing running the program**, `pane_not_found` or `server_not_running`, as its other operations read a pane gone. Not taken: herdr's words as unknown, which would keep a gone agent's start unknown until the stage's limit.

## Amendment, 2026-11-18: how the core reads an agent from its terminal

Status: accepted, the direction unchanged. The choices below are the construct stage's of the change `the-common-profile-reads-an-agent-from-its-terminal`, made on the rules written and cheap to change, each written with the path not taken.

Deciders: the change's construct stage.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clauses 31, 38, 89 to 93, 134, 136, 171 items 2 to 11, 174, C.4's common profile, C.5's herdr send and state rows, and S73; herdr 0.9.5's `agent prompt --help` (`--wait` "requires an observed working or blocked state within 5000ms; otherwise it returns agent_prompt_stalled"); `crates/flume-seams/src/agent.rs` (`Part`), `terminal.rs` (`send`), `held.rs` (`StartHeld`); `crates/flume-core/src/reading.rs` (`read_agent`, `state_of`, `read_out`), `deliver.rs`, `acts.rs` (`hand`), `line.rs`, `settle.rs` (`step`), `pending.rs`; the change's design, `openspec/changes/the-common-profile-reads-an-agent-from-its-terminal/design.md`.

### Decision outcome

- **Received, asking, own work, ended and last words join the parts**, before spent, in 171's order, each with how flume reads it from the terminal; Claude Code's profile builds all thirteen, and usage and plan still join with the changes that add them to the seam (152, 164). Not taken: one part standing for the five, which would leave a profile that reads its program's asking but not its own work no way to say so.
- **The core reads a part a profile does not build from the agent's terminal, in one reading per part**, and asks the profile nothing of it. Not taken: the profile reading the terminal, which crosses the seams (core.2).
- **A prompt and a tell are sent by a new terminal operation, `send_watching`, which answers whether the terminal saw the turn the sending began**, refused by default naming no terminal; and a reading of the agent working or asking within 60 seconds of the sending counts as the turn seen too, since the agent was read waiting and not asking when it was sent. Not taken: a flag on `send`, which would change every terminal and fake for one profile's need; and the reading alone, which misses a turn that ends between two readings.
- **Own work is read from the start's held `waiting`, when a settle first and last read the agent waiting**, written by each settle's step for a live start whose profile reads no own work, cleared at any other reading and at every sending, and read by status, the list and a wait without writing it; and a stopped agent's own work reads none. Not taken: a time kept beside the held state (core.16); a reading by status counting as a settle, which 171 does not read; and own work running for a stopped agent, which would keep a start begun otherwise from ever being ended (31, 161).
- **The place's line reads such an agent's asking from the held `asking_since`**, which the settle writes. Not taken: the line reading the terminal, which a line asked alone and the list may not (core.52).
- **The clause list builds 171 and S73**; 173, S71 and S72 stay named for the change of a pool's program. The statement holds as written: 171 and C.5's herdr send row already name the watched prompt, so no amendment is made.

## Amendment, 2026-11-18: how a pool's account names a stage's program

Status: accepted, the direction unchanged. The choices below are the construct stage's of the change `a-pool-of-several-programs-picks-each-stages-program`, made on the rules written and cheap to change, each written with the path not taken.

Deciders: the change's construct stage, and its code stage for the last three choices.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` at 48a94da, clauses 32 item 8, 144 to 147, 153, 169, 172, 173, 179, 196, 197, 207, 213, S71 and S72; `crates/flume/src/profiles.rs` (`DEFAULT_PROGRAM`, `defaults`); `crates/flume-core/src/settings.rs` (`resolve`, `resolution`, `kind_started`), `accounts.rs` (`PoolRoom`, `choose_account`, `affinity_after`, `pool_runs`), `start.rs` (`claim`), `room.rs` (`Waiting`, `Queue::runs`, `Queue::wants`), `launch.rs` and `acts.rs` (`prompt`); `crates/flume-seams/src/store.rs` (`Affinity`); `crates/flume-conformance/src/clauses.rs` (`PROFILES`, `PLAN`); the change's design, `openspec/changes/a-pool-of-several-programs-picks-each-stages-program/design.md`.

### Decision outcome

- **The account names the program at the layer `account`, and flume's default names one only with no pool** (32 item 8): where only the default names a stage's program and its stream has a pool, the chosen account's program stands in its place, and status gives it from `account`, as it gives the account's variables. Not taken: the program dropped from the engine's defaults and filled at each start with no pool, which would leave status no layer to give; and a new layer, when the program is the account's as its variables are.
- **The programs that can run a stage are read by the start's own checks**, once for each program of the pool's accounts: a profile binds it, and its mode, model, effort and own settings are taken (173). Not taken: a check of its own beside the start's, which could drift from what a start refuses.
- **The terminal's start is asked after the choice**, under the chosen account's environment, refusing the stage in its words as for a program a layer names (32, C.5). Not taken: the terminal among what makes an account usable, which 173 does not list and which herdr answers only by a call under each account's environment while the room lock is held.
- **A stage that names no program takes the affinity's account for the program its last agent ran while it can run the stage, then the strategy's choice over every account that can run it**; the intent moves for that program only where the strategy chose an account of it, and a start on another program moves nothing (147). The affinity holds `last`, written at every start with a pool. Not taken: the affinity's account for every program that can run the stage, which 147 does not say; and the strategy preferring the last program's accounts, when 147 sends the start where 145 chooses.
- **A conversation goes to no other program**: a stopped, lost or hibernated start's profile is asked whether its conversation carries only where the account chosen runs its program, and otherwise it stays, with words saying so (146, 179, 213). Not taken: asking another program's profile of the conversation.
- **The queue, the launch and the prompt's preview read the same programs**: a waiting stage's want counts only accounts that can run it, a launch is refused where none can, naming each program and why, and the preview fills with the profile of the account a start would choose now, writing nothing (4, 29, 145; core.8, core.26). Not taken: reading them at the claim alone, which would count a want from accounts that cannot run the stage; and flume's default profile for every preview.
- **The clause list builds 173 and S71; S72 is named for record 0024's plan stage's change**, beside 163, 164 and S68, since both its halves launch a plan stage; that change also adds the plan to what makes an account able to run a stage that delivers a plan (164, 173). The statement holds as written: 173's plan condition holds for no stage until a stage delivers a plan. Not taken: building the plan part here, which is that change's whole.
- **`live` is asked of the program of the start whose conversation it is**, at a start that carries or follows a stopped, lost or hibernated conversation and at each settle while a stage waits on that conversation's process, since that program's profile alone reads its sessions (179, 207). Not taken: the program of the account chosen, whose profile would read another program's session as none running.
- **A stage waiting for an account is filled, before it waits, with the profile of the first program of its pool that can run it**, as the preview is where no account is usable; and a waiting stage whose settings cannot be read holds no programs, so its want is read as before and its claim refuses it (29, 173). Not taken: refusing such a stage ahead of room at the next settle, which would refuse it for a reading the host's configuration may mend before its room comes.
- **The prompt's preview reads the intent's affinity without removing one past its hour**, which a start's read removes, so the preview writes nothing (119, 147). Not taken: the start's read, which writes.
