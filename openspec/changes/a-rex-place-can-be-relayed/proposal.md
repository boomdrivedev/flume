# A Rex place can be relayed

## Why

A caller that carries a person's screen and keys, as Boomdrive's runner
does for its cloud, reaches a place's live agent by `flume relay` (221).
Under herdr it does. Under Rex it is refused, because C.5 gave Rex no
relay row, and flume's Rex profile refuses relay before any call
(record 0070, core.19). So a person cannot reach an agent of a stream
whose multiplexer is Rex from elsewhere.

The statement now gives Rex a relay row: C.5's Rex relay row, on
boomdrivedev/blueprints at 3d0200b, merged to main at cb114fd, from
boomdrive-ops' reading of Rex 0.1.0-preview.14 on deneb on 2026-11-14.
It is amended at e28b901 and bf9e16f, on the branch
design/rex-relay-types-what-it-can, which waits on the user's push. The
amendments follow this change's readings of the pinned Rex's keys on
2026-11-15 and the design's rulings on them. The block's screen is its
last rows of `capture --format vt --trim`, with its colours. Text goes
by `send` and keys by `send-key`, a key only where the pinned Rex types
it as the person typed it. A character `send-key` refuses or types as
another, as it types a capital as its small letter, goes in as text
where bare, and is refused where led, as is every led key it types
otherwise than typed. Read beside 221 and 222, Rex's relay is built as herdr's is.
The queued unit is flume's batch 12, after
flume-relays-a-places-terminal, on the bolt a-rex-place-can-be-relayed.

## What Changes

- **Rex's relay (C.5's Rex relay row).** The Rex profile's relay finds
  the agent's block from its record and builds a handle. Its bound is
  250 milliseconds. It reads by `capture -s <session> -b <block>
  --format vt --trim` and `block call size`, and gives the capture's
  last `rows` lines as Rex wrote them, the block's screen and never its
  scrollback. It types text by `send -s <session> -b <block> --
  <text>`, in pieces of at most 60,000 bytes, and keys by `send-key -s
  <session> -b <block> <key>…`, by flume's names, which are Rex's own.
  It clears nothing and reads no receipt. A key goes to `send-key` only
  where the pinned Rex was read to type it as the person typed it. A
  character its `send-key` refuses, or types as another, as it types a
  capital as its small letter, goes in as text by `send` where it is
  bare. Where it is led, it is refused, named, with nothing sent for
  it, and the keys around it are typed (222). So is every led key the
  pinned Rex types otherwise than typed: `ctrl+shift+` and a character,
  `ctrl+` and a character whose control it types as nothing, and
  `alt+shift+` and a character whose shift it drops. A block Rex holds no more, or a server
  not running, reads gone. Each call is bounded at one second, as
  herdr's are. A refusal's words name the typed text `<typed>`.
- **No more refusal under Rex.** The Rex profile's refusal of relay and
  its `NO_RELAY` go. Its tests now say it relays, so the terminal
  seam's relay scenarios run against the pinned Rex.
- **The stand-in under Rex shows what it holds typed**, as Claude
  Code's input does, so a relay's typing shows on its screen. It also
  runs `cat` on the test's word, so every key name can be checked
  against the pinned Rex.
- **Through the binary.** A walkthrough relays a fix's agent under Rex
  with `flume relay`, as the herdr one does.
- **Record 0070's amendment and rule core.57's details**, proposed, say
  how Rex relays. core.57's details drop Rex from the terminals that
  offer no relay and add Rex's bound, its keys and its typed text.
- **The site** gives Rex's relay in `flume relay`'s reference, and its
  rules page mirrors core.57.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `terminal-profiles`: Rex relays a block, and no longer refuses relay.
- `place-relay`: a place whose agent runs in Rex is relayed through the
  binary as one under herdr is.

## Impact

- `crates/flume/src/profiles/terminal/rex/mod.rs`: the relay handle,
  `relay`, the words of a refusal, and the module's doc; its unit tests.
- `crates/flume/tests/profiles/tools/terminal_rex.rs`: the stand-in,
  the harness's `relays`, and the key-name check against the pinned Rex.
- `crates/flume/tests/profiles/tools/walkthroughs.rs`: the walkthrough
  under Rex.
- `docs/adr/0070-…` (amended), `docs/architecture/core.md`,
  `site/src/content/docs/architecture/rules.md`,
  `site/src/content/docs/reference/commands.md`.
- No change to the seam, the core, the command or the fakes: the
  relay's loop, its lines and its ends are the ones
  flume-relays-a-places-terminal built.
- Boomdrive's runner relays a Rex place by the same `flume relay` with
  no change of its own.
