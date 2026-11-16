## ADDED Requirements

### Requirement: Rex relays an agent's block

The Rex profile's relay SHALL find the agent's block from its record,
making no Rex call, and refuse with `no agent runs under <agent>` where
no record names a block. Its handle SHALL state 250 milliseconds and
read by `capture -s <session> -b <block> --format vt --trim` and `block
call size`, giving the capture's last `rows` lines as Rex wrote them,
never read as JSON (136, 221, 222, C.5).

#### Scenario: Reads over a stand-in rex

- **WHEN** the profile relays an agent whose record names the block `block:b1` in the Rex session `s1`, over a stand-in `rex` that logs its arguments, answers `size` with 3 rows and `capture` with five lines, each opening with `ESC[0m` and `ESC[38;5;1m` and ending `\r\n`, and the handle reads twice
- **THEN** the handle states 250 milliseconds, the stand-in ran `capture -s s1 -b block:b1 --format vt --trim` and `block call -s s1 -b block:b1 size` at each read, each read gave the last three lines exactly as the stand-in wrote them, their `\r\n` and colours kept, and dropping the handle called nothing (136, 222, C.5)

#### Scenario: A screen that reads as JSON

- **WHEN** the stand-in `rex` answers the relay's `capture` with `{"a": 1}` and a newline
- **THEN** the read gives `{"a": 1}` and the newline, as written (222)

#### Scenario: No record of the agent

- **WHEN** the profile is asked to relay an agent no record names
- **THEN** it is refused with `no agent runs under` and the agent's identity, and no Rex call is made (4, 222)

#### Scenario: The terminal's scenarios against the pinned Rex

- **WHEN** the terminal seam's scenarios run against the Rex profile and the pinned Rex, its harness saying it relays, its stand-in showing what it holds typed
- **THEN** each passes, the relay's among them (221, 222, C.5)

#### Scenario: A block's screen and not its scrollback against the pinned Rex

- **WHEN** a block of the pinned Rex with 24 rows has printed 60 numbered lines, each in colour, and the profile's relay reads it
- **THEN** the read holds line 60, no more lines than the block has rows and not line 1, and each line it holds keeps its colour (136, 222, C.5)

### Requirement: Rex's relay types as a person typed

The Rex profile's relay handle SHALL type text by `send -s <session> -b
<block> -- <text>`, in pieces of at most 60,000 bytes cut between
characters, and each run of keys `send-key` takes by one `send-key -s
<session> -b <block> <key>…` by flume's names. It SHALL clear nothing,
press no Enter of its own and read no receipt (95, 107, 221, 222, C.5).

#### Scenario: Types over a stand-in rex

- **WHEN** the relay of the block `block:b1` in the Rex session `s1` types the text `yes`, then the keys `ctrl+c` and `enter`, over the stand-in `rex`
- **THEN** the stand-in ran `send -s s1 -b block:b1 -- yes` and then `send-key -s s1 -b block:b1 ctrl+c enter`, and nothing else (95, 222, C.5)

#### Scenario: A long text

- **WHEN** the relay types a text of 150,000 bytes, non-ASCII among them, over the stand-in `rex`
- **THEN** the stand-in ran three `send` calls, each piece at most 60,000 bytes and cut between characters, their text together the whole text in order, and no `send-key` (95, 222, C.5)

### Requirement: Rex's relay types a character send-key does not type as itself as text, and refuses it led

The Rex profile's relay SHALL type a character the pinned Rex's
`send-key` names none of, the nineteen of design section 4 and every
character outside ASCII, or types as another, each capital, as the text
it is by `send` where it is bare. Where it is led, the relay SHALL
refuse it, named as a key Rex 0.1.0-preview.14 cannot type, sending
nothing for it. The keys around either SHALL go in, in order (222, C.5's
Rex relay row at blueprints bf9e16f; core.29).

#### Scenario: Keys and characters among keys over a stand-in rex

- **WHEN** the relay types the keys `a`, `!`, `?`, `b`, `ctrl+é` and `enter` over the stand-in `rex`
- **THEN** the stand-in ran, in that order, `send-key -s s1 -b block:b1 a`, `send -s s1 -b block:b1 -- !?`, `send-key -s s1 -b block:b1 b` and `send-key -s s1 -b block:b1 enter`, nothing for `ctrl+é`, and the input is refused naming `ctrl+é` as a key Rex 0.1.0-preview.14 cannot type (222, C.5)

#### Scenario: Every key name against the pinned Rex

- **WHEN** the relay types alone into a block of the pinned Rex running `cat` in raw mode, its extended input off, each of flume's named keys, `f1` to `f12`, every printable character of ASCII and `é`, each bare and under each of `ctrl+`, `alt+` and `shift+`, their pairs and all three, then the keys `a`, `?` and `b` together, then `a`, `ctrl+?` and `b`, then `A`, `ctrl+shift+a` and `b`
- **THEN** each key the relay hands `send-key` types something, a bare character types itself, `alt+` and a character or `space` types ESC before what the key without `alt+` types, and `shift+` and a small letter types its capital. Each key the relay refuses is named as a key Rex 0.1.0-preview.14 cannot type and types nothing. The pinned Rex's own `send-key` still refuses each of the nineteen and `é`, still types each capital as its small letter under the same leads, and still types each other key the relay refuses as nothing or as that key with a lead dropped. `cat` reads `a?b` for the first three keys, `ab` for the next, `ctrl+?` refused, named, and `Ab` for the last, `ctrl+shift+a` refused, named (222, C.5; core.29)

### Requirement: Rex's relay refuses a led key the pinned Rex types otherwise than typed

The Rex profile's relay SHALL hand `send-key` only a key the pinned Rex
was read to type as the person typed it. It SHALL refuse each other led
key, named as a key Rex 0.1.0-preview.14 cannot type, sending nothing
and building no escape sequence for it, and the keys around it SHALL go
in, in order (222, C.5's Rex relay row at blueprints bf9e16f; core.29).

#### Scenario: Capitals and keys Rex types otherwise over a stand-in rex

- **WHEN** the relay types the keys `A`, `b`, `ctrl+shift+a`, `alt+shift+a`, `ctrl+i`, `shift+A` and `enter` over the stand-in `rex`
- **THEN** the stand-in ran, in that order, `send -s s1 -b block:b1 -- A`, `send-key -s s1 -b block:b1 b` and `send-key -s s1 -b block:b1 enter`, nothing for the four led keys, and the input is refused naming each of `ctrl+shift+a`, `alt+shift+a`, `ctrl+i` and `shift+A` as a key Rex 0.1.0-preview.14 cannot type (222, C.5)

### Requirement: Rex's relay reads a gone block gone

The Rex profile's relay handle SHALL read gone where Rex answers that
it holds no such block, `block target "<block>" not found`, or that no
Rex server runs at the block's socket. It SHALL read unread, in Rex's
words, where Rex refuses otherwise (96, 222, C.5).

#### Scenario: A block not found

- **WHEN** the stand-in `rex` answers the relay's `capture` with `Error: block target "block:b1" not found` and exit 3
- **THEN** the read is gone (222)

#### Scenario: A server not running

- **WHEN** the stand-in `rex` answers the relay's `capture` with `Error: no Rex server is running at` the socket
- **THEN** the read is gone (222)

#### Scenario: Another refusal

- **WHEN** the stand-in `rex` answers the relay's `capture` with `Error: busy` and exit 1
- **THEN** the read is unread with `busy` in its words (98, 222)

### Requirement: Rex's relay calls are bounded, name their server and repeat nothing typed

Each call of the Rex profile's relay handle SHALL be `rex
--autostart=false -S unix://<socket>` to the block's own server, with
the caller's `REX_*` stripped, and bounded at one second. A call past
the bound SHALL be killed, a read unread and an input refused, each
naming the bound. No words of the profile SHALL repeat a `send`'s text,
which they name `<typed>` (98, 221, 222; core.29, core.57).

#### Scenario: Each call names the block's server

- **WHEN** the relay of a block at the socket `S` reads once and types the text `yes`, over a stand-in `rex` that logs its arguments and environment, the calling process's `REX_BLOCK` and `REX_SERVER` set
- **THEN** each call the stand-in ran is led by `--autostart=false -S unix://S` and its environment holds no `REX_` variable (core.29)

#### Scenario: A read that does not answer

- **WHEN** the stand-in `rex` sleeps five seconds before answering a relay's read
- **THEN** the read is unread naming one second, within two seconds of being asked (98)

#### Scenario: A text that does not answer

- **WHEN** the stand-in `rex` sleeps five seconds before answering a relay's `send` of `secret-typed`
- **THEN** the input is refused naming one second and `<typed>`, within two seconds, and its words do not hold `secret-typed` (98, 221)

## REMOVED Requirements

### Requirement: Rex refuses relay

**Reason**: C.5 now gives Rex a relay row (boomdrivedev/blueprints 3d0200b, on main at cb114fd, as e28b901 and bf9e16f amend it), so the Rex profile relays rather than refusing before any call.

**Migration**: Rex's relay is the requirement "Rex relays an agent's block" and the four beside it. The Rex harness says it relays, so the terminal seam's relay scenario runs against the pinned Rex in place of the refusal.
