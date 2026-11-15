# Design

The statement is read at boomdrivedev/blueprints main cb114fd, with
C.5's Rex relay row as e28b901 amends it, on the branch
design/rex-relay-types-what-it-can, which waits on the user's push. It
also reads 107, 221 and 222, 222 as b54a9b4 amended it, and the Rex
paragraphs and rows of C.5 the relay row leans on: the screen row's
last rows, the send row's pieces, the state row's stopped, the bound on
every call, and what Rex 0.1.0-preview.14 was seen to do. The relay
itself is flume-relays-a-places-terminal's, and record 0070 holds it.
This change builds Rex's half of the terminal seam's relay (222). Where
this design and the statement disagree, the statement wins (core.19).

The rules relied on: core.1 and core.2 (Rex's commands are the
profile's, reached through the terminal seam's relay alone); core.11
(a relay's keys are no send, so nothing is cleared and no receipt read);
core.13 (the relay lives inside its command); core.16 and core.56
(nothing of a relay is kept or told); core.29 (every Rex call bounded,
`--autostart=false` and the session's socket, the caller's `REX_*`
stripped, each command and field checked against the pinned Rex);
core.30 (no variable's value in words); core.35 (the site speaks to any
host's operator); core.49 (the relay goes to the terminal the agent's
start recorded); core.57 (how a relay goes and what it keeps). No area
page exists, so core.md binds every path this change touches.

## 1. Scope

This change builds the Rex profile's relay and nothing else of flume's.
The seam's `Relay`, `Typed`, `Key` and `RelayScreen`, the core's loop,
`flume relay`, its lines, its ends and the fakes stand as
flume-relays-a-places-terminal built them. The core reaches Rex's relay
as it reaches herdr's, through `terminal_of` and the seam (190;
core.49). A Rex place's relay ends when the core reads the block
stopped at the first tick of a second, by the state row, or when a read
answers gone. The row's "it ends ... once state reads the block
stopped" is that.

## 2. The ask (222)

`relay(agent)` reads the agent's record, as send and screen do, and
makes no Rex call. With no record, or a claim whose block is not yet
recorded, it refuses with `no agent runs under <agent>`, the words send
gives. Otherwise it builds the handle over the recorded block, its
socket and its Rex session where the record names one. A block that is
gone by the time of the ask reads gone at the handle's first read, and
the core ends the relay gone (221).

Rex keeps a block's id when it moves between windows, and no Rex call
finds a block by its label. So, unlike herdr's, the handle never finds
the block again: the id it was given is the one every call names.

## 3. Reading the screen (C.5's Rex relay row)

- `every()`: 250 milliseconds.
- `screen()`: `capture [-s <session>] -b <block> --format vt --trim`,
  then `block call [-s <session>] -b <block> size`, and the capture's
  last `rows` lines. Rex's vt capture holds the block's scrollback with
  its screen. Each line opens with Rex's own `ESC[0m`, sets its own
  colours and ends `\r\n`, and no cursor position is carried. So the
  cut keeps the colours, and a person's view, the block's screen, is
  what is given, never the whole scrollback (222).

The capture's standard output is taken as Rex writes it and never read
as JSON, so a screen whose text parses as JSON is not turned into
something else. The cut keeps each line it gives as Rex wrote it, its
`\r\n` among it, rather than joining lines by `\n` as the screen row's
`last_rows` does for status. A `rows` Rex does not give is 24, as the
screen row reads it. `size` is read at each read, as the screen row
reads it, so a block a person resized is cut to its new rows.

A refusal of either call naming `block target "<block>" not found`, or
no server running at the socket, reads `Gone`. Any other refusal is
`Unread` in Rex's words, and a call past its bound is `Unread` naming
one second.

## 4. Typing (C.5's Rex relay row, 222)

Text goes by `send [-s <session>] -b <block> -- <piece>`, each piece at
most 60,000 bytes cut between characters, as the send row cuts them, in
order. Nothing is cleared first, no Enter of the profile's own is
pressed and no receipt is read (95, 221). A text Rex refuses part of the
way is refused in Rex's words, and the pieces before it stand typed, as
a person's typing that stopped would.

Keys go by flume's names, which are Rex's own, `send-key [-s <session>]
-b <block> <key>…`. The pinned Rex takes several keys in one call and
types them in order. It takes every named key of flume's, `f1` to
`f12`, the letters, the digits and `'`, `+`, `,`, `-`, `.`, `/`, `;`,
`=`, `>`, `[`, `\`, `]` and the backtick, each bare or led by `ctrl+`,
`alt+` and `shift+`. It refuses the nineteen other printable characters
of ASCII, `!`, `"`, `#`, `$`, `%`, `&`, `(`, `)`, `*`, `:`, `<`, `?`,
`@`, `^`, `_`, `{`, `|`, `}` and `~`, and every character outside
ASCII, and types nothing of a call that holds one. The row, as e28b901
amends it, says what the relay does with them:

- **Bare**, such a character goes in as the text it is, by `send`. A
  person typing it bare types exactly that text, and Rex types it by
  `send`.
- **Led** by `ctrl+`, `alt+` or `shift+`, it is refused, named as a key
  Rex 0.1.0-preview.14 cannot type, and nothing is sent for it.

So a relay's keys go in order as runs. Each run of keys Rex takes is one
`send-key`. Each run of bare characters it refuses is one `send` of
their text, cut as any text is. Each led one is refused, ending the run
before it. The input is then given back as untyped, naming each key not
typed, as herdr's relay gives back its six. The profile holds the
nineteen as a list beside herdr's `CANNOT_TYPE`, and any character
outside ASCII by its range. It does not learn them from Rex's refusal,
which would drop the whole call. The tools layer reads the pinned Rex
taking every other name and still refusing each of these by
`send-key`, so a Rex that comes to type them shows there.

## 5. The calls (98; core.29)

Every relay call is the profile's own `rex --autostart=false -S
unix://<socket> <words>` with the caller's `REX_*` stripped, to the
block's own server. Each is bounded at one second rather than the
profile's ten. Rex answers a local call in milliseconds, and a read
that waits holds the person's screen, as record 0070 chose for herdr. A
relay run from inside another Rex block reaches nothing of that block's
server.

## 6. Nothing kept, no words of what was typed (221; core.57)

The handle writes no record, file, log line or telemetry. The profile's
words for a call, used where Rex prints nothing or a call passes its
bound, give `send`'s text after `--` as `<typed>`, for every `send`, a
prompt's pieces and a bare character among them. So a refusal or an
unread read never repeats what a person typed, as herdr's `unvalued`
names `pane send-text`'s text. Keys are named in words as herdr's are,
since a key's name is what a refusal of it must give (222). Dropping
the handle ends and closes nothing in Rex (221).

## 7. The stand-in under Rex

The terminal seam's relay scenario types text onto what the agent
holds typed and asks that it show on a screen read within ten seconds.
It then asks that `enter` submit both together, and that a closed
agent's read be gone (seam-contracts). The Rex tools' stand-in reads its
terminal unechoed, a chunk at a time, as Claude Code does, and so shows
nothing typed. It gains a line that shows what it holds typed, drawn
again after each chunk it reads, as Claude Code draws its input box.
Each line it takes and each `print` stay as they are, so the Rex tests
that read the screen read what they did. It also gains `STAND_IN_CAT`,
as herdr's stand-in has: once the test says go, it runs `cat` in raw
mode into its log, so every key and character a relay types reaches it
as bytes.

## 8. Through the binary (221)

A walkthrough under Rex mirrors the one under herdr. A fix is launched
on the stream `trial`, whose multiplexer is Rex, with the stand-in agent
program, which runs `stty echo < /dev/tty` before it waits. Once its
agent has said it started, `flume relay fix--one` runs as a child. Its
first line is a screen. `{"text":"relay-marker"}` is written to its
input, and a later screen holds `relay-marker`. Once its input closes,
its last line is the relayed answer ended by the caller and it exits 0.
The place's held state stands at its version, and no file under the
host's state directory holds `relay-marker` (107, 221, 222, C.5).

## 9. The record and the rules

Record 0070 gains an amendment of 2026-11-15, proposed: a Rex place is
relayed. Its decision 7, "Rex refuses", is replaced by C.5's Rex relay
row at e28b901, quoted. The amendment holds the pinned Rex's reading of
its keys, and the design's ruling of 2026-11-15 on this change's
construct stop-short, quoted. The ruling gave the two choices the row
now makes, against the construct's recommendations:

- a bare character `send-key` refuses goes in as text by `send`, and
  only a led one is refused; not every such character refused, bare or
  led;
- the block's last `rows` lines; not the whole capture.

The construction agent's own choices are recorded with the path not
taken:

- the ask makes no call, not a `process` call first;
- the capture is taken as written, not read as JSON;
- the cut keeps Rex's line endings, not `last_rows`' `\n`;
- `size` is read at each read, not once at the ask, which would miss a
  resize;
- each run of keys is one `send-key`, and each run of bare refused
  characters one `send`, not one call a key;
- the calls are bounded at one second, not the profile's ten;
- `send`'s text is named `<typed>` for every send, not for the relay's
  alone;
- the characters Rex refuses are a list the profile holds, not learned
  from Rex's refusal of a whole call.

core.57's own text stands, since it already says a key the terminal
cannot type is refused, named, and the keys after it typed. Its details
change:

- "Not ruled out" no longer names Rex among the terminals that offer no
  relay. It gains a bare character Rex's `send-key` refuses typed as
  text by `send`.
- "Rules out" gains:
  - a Rex relay call with no bound or one of more than a second;
  - a Rex relay giving more than the block's last `rows` lines;
  - `send`'s text repeated in words of flume's;
  - a character Rex 0.1.0-preview.14's `send-key` refuses sent to
    `send-key`;
  - such a character given led sent at all;
  - the keys around a refused one dropped with it.

The source list gains C.5's Rex relay row at e28b901. The site's rules
page mirrors core.57 as written.

## 10. The suite

The Rex profile's unit tests run over a stand-in `rex` that logs its
arguments and environment and answers as the test scripts it, as the
herdr ones run over a stand-in `herdr`. The tools tests run against the
pinned Rex under a state folder of their own, each server stopped at
the end (core.51). The conformance suite's clause list stays at 81288ba
as it stands, where 221 and 222 are listed but not yet built: this
change adds no scenario to the conformance suite. The Rex harness saying
it relays runs the seam's relay scenario, which cites only the clauses
that list holds.
