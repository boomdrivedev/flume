---
number: 78
title: A watch settles only the places where something is pending
status: accepted
date: 2026-11-17
decision-makers:
- Chuck Swanberg
- boomdrive-design
---

# A watch settles only the places where something is pending

Status: accepted by the user on 2026-11-17, on their card.
You chose on your card today that Flume should look only at places where something is pending, instead of going through every place every minute. This record writes that down, with three details I settled. Flume, not the runner, works out which places are pending, so the runner keeps the call it makes today. The "still watching" signal is that same short look, not a separate ping, because a ping from a stuck runner would claim the machine is watched while nothing moves. And on a machine where nothing keeps watch, each Flume command also looks only at pending places, so it takes seconds, not minutes. Say yes if this is what you meant.

Deciders: the user, on their Pending You card `req_64be542cc8ac4aa38180` of 2026-11-17, choosing to look only where something is pending over keeping the sweep and making it safe; boomdrive-design, on what counts as pending, who reads it, what renews the watch and what an unwatched command settles.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md`, clauses 46, 119, 208 and 225, on the branch `design/flume-settles-what-is-pending` at 48a94da; record 0051, whose watch this amends in part; rule core.13; flume's settle as built on main at a2535f29 (`crates/flume-core/src/settle.rs`: `settle_every`, `settle_all`, `settle_attended`, `settle_place`, `step`); the fix `a-settle-reads-sluice-only-for-a-submission-in-flight`, landed at ed80a01c; deneb's readings of 2026-11-17, where Flume commands stalled 190 to 300 seconds.

## Context and problem statement

Flume runs no process of its own (120). A place moves on only when a settle reads it, and a settle runs only inside a command. Three things that move a place call Flume when they happen: an agent going quiet (121), a command stage's command exiting (223), and a caller's command. Some things call nothing: the merge queue deciding a submission, a time a place waits on coming, an agent dying with no word. Something has to look for those.

Record 0051 gave that look to a caller keeping watch: `flume settle --every <bound>` settled every place on the host and kept the watch while its newest settle began less than twice its bound ago. Where no caller kept watch, every command of Flume's that acted settled every place first (119).

On deneb a settle of every place read each place in full, its terminal among them, and the merge queue for each. A pass ran minutes, outlived its own watch, and every command then settled every place itself: the three-to-five-minute freezes. Statement 208 was amended on 2026-11-17 so a watch holds while its settle runs. That stops the lapse, but the pass still grows with every place on the host, and most places it reads have nothing that could have moved them.

## Considered options

For what a watch settles:

1. **Only the places where something is pending (chosen by the user).** A place is read only where a change that calls Flume nothing may have moved it.
2. **Every place, made safe.** Keep the sweep; the watch holds while it runs. One small fix, and a pass that still grows with the host.
3. **Sluice calls Flume when it decides.** Not offered: no tool calls another to say what happened; each writes its record and the next one reads it.

For who works out what is pending:

1. **Flume (chosen).** The caller's command stays `flume settle --every <bound>`, and a caller keeping watch decides nothing of what a settle does (208). A host's own timer can keep watch as well as the runner can.
2. **The caller.** The runner would read Flume's deadlines and the merge queue and settle named places. Every caller would then carry Flume's knowledge of its places, and a plain timer could keep no watch.

For what tells Flume the host is watched:

1. **The watch's settle itself (chosen).** The watch holds while that settle runs and until twice its bound after it ended. A host reads watched only while its places are being settled.
2. **A heartbeat apart from the settle.** Cheap, but a runner whose settling is stuck while its heartbeat goes on would leave the host reading watched while no place moves.

For a command where no caller keeps watch:

1. **It settles the pending places first (chosen).** The same reading as a watch's, so an unwatched host waits seconds, not minutes.
2. **It settles every place first, as before.** A lapsed watch would bring the freezes back.

## Decision outcome

- **A watch settles only where something is pending.** Something is pending on a place while a change that calls Flume nothing may have moved it: it is being made, or its work is over and it is not yet tidied away; a stage of it starts or runs; its submission is in flight; it is a base place, whose base may have moved; a time it waits on has come; it waits for room the host now has; or an end of it, or a wake for it, waits to be handed on (225). A place that waits only on an answer, its caller's word or a drop has nothing pending.
- **Nothing moves later than before.** A settle of the pending places moves every place no later than a settle of every place would have. A place with nothing pending has its merge queue, its terminal and its agent read by no such settle.
- **Flume alone reads what is pending.** The caller names nothing of it, and `flume settle --every <bound>` keeps its form, so the runner's call is unchanged.
- **The watch is renewed by its settle alone**, held while that settle runs and until twice its bound after it ended (208).
- **A command where none keeps watch settles the pending places first** (119).
- **A full settle stays at hand.** `flume settle` naming no place and stating no bound still settles every place, at a caller's or a person's asking.

### Consequences

- Rule core.62 holds the outcome. Record 0051's watch reads as amended here.
- The runner needs no change for this record. Its line on a refused settle names "every place", which a later change of the runner's may word anew.
