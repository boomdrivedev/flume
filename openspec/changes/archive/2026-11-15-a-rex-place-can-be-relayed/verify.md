# Verify: a-rex-place-can-be-relayed

Verified on `change/a-rex-place-can-be-relayed` at 462cb558, cut from
`bolt/a-rex-place-can-be-relayed` (4dd68fdf).

**Outcome: the whole checks pass and the work holds; the change is
archived.**

## The whole checks

`devenv shell -- sluice exec bolt/a-rex-place-can-be-relayed`, run once
with sluice 0.93.2 (the devenv's, the same release as the host's): exit
0. sluice ran the checks 462cb55 affects against the base at 4dd68fd,
the bolt's required `#check-fast`: 13 tasks completed, 12 results
recorded under their keys for 462cb55, none failed. They were
`flume:format`, `flume:lint` and `flume:test`; `conformance:test`;
`nix:typecheck` and `nix:test`; `site:install`, `site:prose` and
`site:build`; `plugin-claude:test`; and `root:format`, `root:changes`
and `root:adrs`. `site:prose` gave suggestions only, on pages this
change does not touch. moon warned it could not archive task outputs
into `.moon/cache/outputs`, which fails nothing.

`openspec validate a-rex-place-can-be-relayed --strict`: valid.

The bolt requires no `#check-tools`, which runs at the bolt's merge
onto main. So the scenarios against the pinned Rex were checked apart
from the gate, by the tests the tasks name. `sluice exec
bolt/a-rex-place-can-be-relayed -- cargo test -p flume --features tools
--test tools` ran with the filters `terminal_rex::relay_`,
`terminal_rex::the_rex_profile_passes_the_terminal_scenarios`,
`terminal_rex::the_screen_is_the_blocks_last_rows`,
`terminal_rex::s43_a_prompt_left_typed_is_cleared_before_each_sending_under_rex`,
`walkthroughs::a_person_reaches_a_fixs_agent_under_rex_through_a_relay`
and `walkthroughs::a_person_reaches_a_fixs_agent_through_a_relay`:
exit 0, 7 passed, none failed, in 88 seconds.

## Tasks

Every task of `tasks.md`, 1.1 to 4.3, is ticked. Every test a task names
exists under that name and passed: tasks 1.1 to 1.6's six unit tests of
`profiles::terminal::rex::tests` in the gate's `flume:test`, tasks 2.1
to 3.1's tools tests in the run above, task 4.1's `root:adrs`, task
4.2's `root:format` and `site:prose`, and task 4.3's `site:prose` and
`site:build` in the gate.

- 1.1: `NO_RELAY` and `relay_is_refused_and_calls_nothing` are gone from
  the Rex profile. The `NO_RELAY`s left in the tree are the seam's
  default and the printed command's. The module doc's relay paragraph
  says what the row at bf9e16f says.
- 1.4: `SEND_KEY_REFUSES`, the nineteen, stands beside the module's
  constants, and `CTRL_TYPES_NOTHING`, the twelve, beside it, each doc
  citing the row at bf9e16f and 222.
- 2.2: the Rex harness's `relays` answers true. Its doc cites the row at
  e28b901, the revision that first gave Rex a relay.

## Requirements and scenarios

| spec | requirement | met by |
|---|---|---|
| terminal-profiles | Rex relays an agent's block | "Reads over a stand-in rex" and "A screen that reads as JSON": `relay_reads_the_blocks_last_rows_as_rex_writes_them`, which also reads 24 rows where `size` gives none; "No record of the agent": `relay_of_an_agent_no_record_names_is_refused_and_calls_nothing`, with no record and with a claim whose block is not yet recorded; "The terminal's scenarios against the pinned Rex": `the_rex_profile_passes_the_terminal_scenarios`; "A block's screen and not its scrollback against the pinned Rex": `relay_gives_the_blocks_screen_and_not_its_scrollback` |
| terminal-profiles | Rex's relay types as a person typed | "Types over a stand-in rex" and "A long text": `relay_types_by_send_in_pieces_and_keys_by_send_key` |
| terminal-profiles | Rex's relay types a character send-key does not type as itself as text, and refuses it led | "Keys and characters among keys over a stand-in rex": `relay_types_a_character_send_key_refuses_as_text_and_refuses_it_led`; "Every key name against the pinned Rex": `relay_types_each_of_flumes_key_names_into_a_block` |
| terminal-profiles | Rex's relay refuses a led key the pinned Rex types otherwise than typed | "Capitals and keys Rex types otherwise over a stand-in rex": `relay_types_a_character_send_key_refuses_as_text_and_refuses_it_led`, which also checks `rex_key` over every capital, the nineteen and characters outside ASCII under each lead, every character `send-key` takes under `ctrl+shift+`, `ctrl+`, `ctrl+alt+` and `alt+shift+`, and `ctrl+shift+space` |
| terminal-profiles | Rex's relay reads a gone block gone | "A block not found", "A server not running" and "Another refusal": `a_relay_reads_a_gone_block_gone_and_another_refusal_unread`, which also reads another block's not found unread |
| terminal-profiles | Rex's relay calls are bounded, name their server and repeat nothing typed | "Each call names the block's server", "A read that does not answer" and "A text that does not answer": `a_relay_call_names_its_server_and_past_one_second_nothing_typed` |
| terminal-profiles | (removed) Rex refuses relay | The requirement and its test are gone. Its scenario "The terminal's scenarios against the pinned Rex" is added again under "Rex relays an agent's block", the harness now saying it relays |
| place-relay | A place whose agent runs in Rex is relayed as one under herdr is | "Through the binary under Rex": `a_person_reaches_a_fixs_agent_under_rex_through_a_relay`, which also reads the agent's terminal `{"profile": "rex", "from": "stream"}` in status. The herdr walkthrough, now sharing `relayed_through_the_binary`, still passes |

How the code meets them (`crates/flume/src/profiles/terminal/rex/mod.rs`):

- `Terminal::relay` reads the agent's record through `located` and makes
  no call. `RexRelay` holds the located block and names its id at every
  call.
- `screen` takes `capture … --format vt --trim` by `Rex::printed`, which
  gives the standard output as written and never parses it. It then
  reads `block call … size` and gives `screen_rows`, the last `rows`
  lines each kept with its `\r\n`.
- `typed` sends text by `send --` in `pieces` of `PIECE` bytes. It sorts
  keys through `rex_key` into runs, one `send-key` for each run of keys
  and one `send` for each run of bare characters. A refused key ends the
  run before it, and the input is given back naming each refused key.
- Every call goes through `Rex::printed` under `RELAY_BOUND`, one
  second, as `rex --autostart=false -S unix://<socket>` with `REX_*`
  stripped. `words` names every `send`'s text after `--` as `<typed>`.

## The record, the rules and the site

- Record 0070 gains "Amendment, 2026-11-15: a Rex place is relayed",
  proposed. It quotes the row at bf9e16f in place of decision 7, gives
  the pinned Rex's reading of its keys, and quotes boomdrive-design's
  two rulings of 2026-11-15, each with the path not taken. It also gives
  the construction agents' own choices, each with the path not taken.
- core.57's note reads "(Record 0070 and its amendment of 2026-11-15,
  proposed.)". Its details drop Rex from the terminals that offer no
  relay, and gain the Rex relay call's bound, the last `rows` lines,
  `send`'s `<typed>`, the keys Rex's `send-key` refuses or types
  otherwise than typed, and bare characters typed by `send`. Its source
  gains C.5's Rex relay row at bf9e16f. The site's rules page mirrors
  core.57.
- `flume relay`'s reference names only the printed command's terminal
  among those that offer no relay. It gives Rex's keys sent as text
  bare and refused led, and the 250 milliseconds, the one-second bound
  and the visible screen under herdr and Rex alike. It names no caller
  but a program that carries a person's screen and keys (core.35).

## Outside this repository

- The statement's row is read at bf9e16f, after e28b901, on
  boomdrivedev/blueprints' branch `design/rex-relay-types-what-it-can`.
  That branch waits on the user's push. Until it lands, this change
  rests on a revision of the row that main does not yet hold.
- The amendment's last consequence says the row at bf9e16f names, of the
  led keys Rex types otherwise than typed, only `ctrl+shift+` and a
  letter. The code refuses the rest under the rule boomdrive-design
  gave, a key handed to `send-key` only where it types as typed. Naming
  them in the row is the design's to do.
