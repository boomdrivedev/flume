---
number: 70
title: A caller relays a place's terminal through flume, a person's keys in and its screen out
status: proposed
date: 2026-11-14
decision-makers:
- Chuck Swanberg
- boomdrive-design
- the change flume-relays-a-places-terminal's construction agent
---

# A caller relays a place's terminal through flume, a person's keys in and its screen out

Status: proposed. The user's choice is boomdrivedev/boomdrive's record 0004, accepted by the user on 2026-11-07 on their card, and its statement is flume's 107, 221 and the printed command's paragraph, merged to boomdrivedev/blueprints' main at `41bd0a2` on the user's word of 2026-11-14. 222 and C.5's relay row are read as boomdrivedev/blueprints `b54a9b4` amends them, on its branch `design/a-relay-refuses-a-key-it-cannot-type`, which waits on the user's push. The choices below that are not the user's are boomdrive-design's ruling of 2026-11-14 and the construction agent's own, made on the rules written and cheap to change.

Deciders: the user, in boomdrivedev/boomdrive's record 0004. Their words: "a combo of #1 and #3 might make the most sense. Remember that Herdr should be a seam for multiplexers. We will want to support TuiOS shortly." boomdrive-design, on the keys herdr 0.9.5 cannot type, ruling on 2026-11-14 on the code stage's stop-short. The construction agent of the change `flume-relays-a-places-terminal`, on the rest.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 4, 56, 95, 96, 98, 107, 120, 136, 190, 221 and the printed command's paragraph at `41bd0a2`, and 222 and C.5's relay row at `b54a9b4`; boomdrivedev/boomdrive's record 0004 and its terminal-relay spec; the change `flume-relays-a-places-terminal`, its design sections 2 to 12; herdr 0.9.5's `pane send-keys`, which refuses `delete`, `insert`, `home`, `end`, `pageup` and `pagedown`, bare and led, with `invalid_key`, read on deneb on 2026-11-14; records 0018, 0065 and 0068; rules core.1, core.11, core.13 to core.17, core.29, core.35 and core.49.

## Context and problem statement

A person may open a launch's terminal from its card in Boomdrive's cloud, and the runner carries that person's screen and keys between the browser and the place's live agent. As built, the runner ran herdr's command line itself, which stands against 107: flume opens, sends to and closes every agent's terminal, and no caller does. It also tied the runner to one multiplexer. The user chose that the runner reach a terminal only through a command of flume's, through flume's terminal seam, and that a relayed hand count as a person's (boomdrive record 0004). The statement now says how (107, 221, 222, C.5). This record holds how flume builds it.

## Considered options

1. **The lines.** JSON lines both ways, whatever `--json` says: `{"screen": …}`, `{"unread": …}` and `{"untyped": …}` out, the answer last; `{"text": …}` and `{"keys": [ … ]}` in. Or raw bytes out and in, or `--json` choosing between them.
2. **The keys.** flume's own key names, `enter`, `tab`, `esc`, `backspace`, `delete`, `insert`, `home`, `end`, `pageup`, `pagedown`, `up`, `down`, `left`, `right`, `space`, `f1` to `f12` and any one printable character, each led by any of `ctrl+`, `alt+` and `shift+` in that order. Or each terminal's names passed through.
3. **The seam.** A handle the core drives, its bound, a read and a type one call each. Or one call that runs the whole relay inside each profile.
4. **The agent's state.** Read once a second of the host's clock, and after a read the terminal did not answer. Or beside every read.
5. **herdr's bound.** One second for each relay call. Or the profile's ten.
6. **Keys herdr 0.9.5 cannot type**, `delete`, `insert`, `home`, `end`, `pageup` and `pagedown`, bare or led. (a) Drop them from flume's names. (b) Keep them, and have herdr's relay refuse each, named, with nothing sent for it and the keys around it typed. (c) Type them through `pane send-text` as the escape sequences an xterm sends.
7. **Rex.** Refuse relay. Or build one from Rex's `capture`.
8. **A settle.** None. Or a settle first, as an act takes one.

## Decision outcome

1. **JSON lines both ways.** A caller tells a screen from an end and from a refusal, and the form only gains fields, as every answer does (core.14). No person reads a relay's frames, so `--json` chooses nothing.
2. **flume's key names.** The caller names no multiplexer's keys, as the user ruled, so the runner stays free of herdr's spelling and TuiOS's to come. The seam carries a key as `Key`, read and written by its name, and each profile maps it to its terminal's own (core.1).
3. **A handle the core drives.** The loop, its ends and its reading of state are written once, in the core, over the fakes. A loop in each profile would put them in every profile.
4. **State once a second.** A read every 250 milliseconds with a state read beside it would double herdr's calls for an end a second later. A read the terminal does not answer reads the state at once, so a gone agent ends the relay without waiting.
5. **One second.** herdr answers in milliseconds, and a read that waits ten seconds holds the person's screen. A read past it is unread, and an input past it untyped, naming the bound (98; core.29).
6. **(b), boomdrive-design's ruling.** The six stay flume's names, which a later herdr may type. Under herdr 0.9.5 each is refused, named as a key herdr 0.9.5 cannot type, with nothing sent for it, and the keys around it are typed, each run of them one `pane send-keys`. The input is given back as untyped with those words (222). Not (a): the names are flume's, not herdr's. Not (c): `pane send-text` may paste an escape sequence as text rather than type it as a key, and C.5 sends keys by `pane send-keys`.
7. **Rex refuses.** C.5 gives Rex no relay row, so flume's Rex profile refuses relay before any call. A relay of Rex is the statement's to add first (core.19).
8. **No settle.** The relay is no act (221). It settles, writes and keeps nothing, and a relay of a place whose held state still reads its agent live after the agent went finds it gone at its terminal at once.

The relay reaches the current stage's live agent through the terminal its start recorded (190; core.49). It writes no held state, host document, file, log line or telemetry, records no act, and keeps nothing it carried but the last screen given, to compare the next with. What herdr is given to type travels as its arguments, and a refusal names `pane send-text`'s text as `<typed>`, so no words of flume's repeat what a person typed. The printed command refuses relay, since it holds no part of a terminal a person can be shown (C.5).

### Consequences

- Rule core.57 says how flume relays and what it keeps of a relay, which is nothing. Rule core.11's details gain that a relay's text and keys, typed as the person typed them, are not ruled out.
- Good: the runner drops its own herdr relay for `flume relay`, and TuiOS needs only a profile's relay.
- Bad: under herdr 0.9.5 a person's Delete, Insert, Home, End, Page Up and Page Down do not reach the agent. The caller is told which key did not go in, and a herdr that comes to type them shows in the tools layer, which still reads the pinned herdr refusing each.
- The suite's clause list stays at `31f9f1c`, as record 0065's construct left it: moving it to main's revision brings 211 to 224 and S121 to S131 with it, each to be cited or named unbuilt, and is a change of its own. The relay's scenarios cite the clauses the list holds and name 221 and 222 in their words.
- Elsewhere: Boomdrive's runner relays through `flume relay` and drops its own herdr calls, a change of its own (boomdrive record 0004).

## Amendment, 2026-11-15: a Rex place is relayed

Status: proposed. C.5's Rex relay row is read as boomdrivedev/blueprints `bf9e16f` amends it, after `e28b901`, on its branch `design/rex-relay-types-what-it-can`, which waits on the user's push; the row itself is `3d0200b`, on main at `cb114fd`. The choices below are boomdrive-design's two rulings of 2026-11-15 and the construction agents' own, made on the rules written and cheap to change.

Deciders: boomdrive-design, ruling on 2026-11-15 on the construct stop-short of the change `a-rex-place-can-be-relayed`, and again that day on its code stage's found. The construction agents of that change, its construct and its code stage, on the rest.

Sources: `boomdrivedev/blueprints:design/flume/requirements.md` 107, 221, 222 as `b54a9b4` amends it, and C.5's Rex rows, its relay row at `bf9e16f`; the change `a-rex-place-can-be-relayed`, its design sections 2 to 10; Rex 0.1.0-preview.14's `capture --format vt --trim`, `block call size`, `send` and `send-key`, read on deneb on 2026-11-15, the last typing each of flume's key names alone, bare and under each of `ctrl+`, `alt+` and `shift+`, their pairs and all three, into a block running `cat` in raw mode with its extended input off; records 0068 and 0070's decision above; rules core.1, core.11, core.13, core.16, core.29, core.30, core.49, core.56 and core.57.

### Context and problem statement

Decision 7 above refused relay under Rex, since C.5 gave Rex no relay row. The statement now gives it one, so a person reaches an agent of a stream whose multiplexer is Rex from elsewhere, as one under herdr, through `flume relay` (221).

### Decision outcome

Decision 7 is replaced. Rex relays as C.5's Rex relay row at `bf9e16f` says:

> `capture -s <session> -b <block> --format vt --trim` and `block call size` (`rows`), its last `rows` lines, as the screen row gives them: a person's view is the block's screen, never its whole scrollback. Each line opens with its own reset, so the cut keeps the block's colors, and no cursor position is carried. It is read again at most every 250 milliseconds and given on only where it differs from the last read. The person's text goes by `send -s <session> -b <block> -- <text>`, in pieces of at most 60,000 bytes as the send row cuts them, and their keys by `send-key -s <session> -b <block> <key>`, each as typed and in order. Rex 0.1.0-preview.14's `send-key` types as itself every one of flume's named keys, `delete`, `insert`, `home`, `end`, `pageup` and `pagedown` among them, `f1` to `f12`, the lowercase letters, the digits and `'` `+` `,` `-` `.` `/` `;` `=` `>` `[` `\` `]` and `` ` ``, each bare or led by `ctrl+`, `alt+` and `shift+`, but types nothing for `ctrl+shift+` and a letter. It types a capital as its lowercase letter, bare or led by `alt+`. It refuses `!` `"` `#` `$` `%` `&` `(` `)` `*` `:` `<` `?` `@` `^` `_` `{` `|` `}` `~` and every character outside ASCII, and types nothing of a call that holds one. So a capital, one of those nineteen or a character outside ASCII, given bare, goes in as the text it is, by `send`. Given led, it is refused, named as a key Rex 0.1.0-preview.14 cannot type, and nothing is sent for it, while the keys around it still go in; `ctrl+shift+` and a letter is refused the same way. It ends at the caller's word, or once state reads the block stopped (221, 222)

The profile's relay finds the agent's block from its record with no call, and refuses with `no agent runs under <agent>` where no record names a block. Its handle states 250 milliseconds, reads by `capture --format vt --trim` then `block call size` at each read, and gives the capture's last `rows` lines as Rex wrote them, 24 where Rex gives none. Text goes by `send` in pieces of at most 60,000 bytes, and keys by `send-key`, each run of them one call. Each call is the profile's `rex --autostart=false -S unix://<socket>`, the caller's `REX_*` stripped, bounded at one second. A block Rex answers not found, or a server not running at its socket, reads gone.

**The pinned Rex's reading of its keys.** Rex 0.1.0-preview.14's `send-key` types every named key of flume's and `f1` to `f12`, bare and under every lead, in a terminal's modifier forms: `ESC [ 1 ; 5 A` for `ctrl+up`, `ESC [ 27 ; 2 ; 13 ~` for `shift+enter`, `ESC [ Z` for `shift+tab`. It types each lowercase letter, digit and each of `'` `+` `,` `-` `.` `/` `;` `=` `>` `[` `\` `]` and `` ` `` as itself bare, as its US layout's shifted character under `shift+`, after ESC under `alt+`, and under `ctrl+` as its control where it has one, `^A` to `^Z`, `^@` for `2` and `space`, ESC for `3`, `^\` for `4` and `\`, `^]` for `5` and `]`, `^^` for `6`, `^_` for `7` and `/`, DEL for `8`, and itself for `0`, `1` and `9`. It does not type as the person typed it:

- each capital, which it takes and types as its small letter, bare and under every lead;
- `ctrl+shift+` with a letter, a digit, one of those thirteen or `space`, which types nothing, and after `alt+` types the key under `alt+` alone;
- `ctrl+` with `i`, `m`, `'`, `+`, `,`, `-`, `.`, `;`, `=`, `>`, `[` or `` ` ``, which types nothing, and after `alt+` types the key under `alt+` alone;
- `alt+shift+` with a character its shift changes, every one of those but `+` and `>`, which types ESC and the character unshifted.

It refuses `!` `"` `#` `$` `%` `&` `(` `)` `*` `:` `<` `?` `@` `^` `_` `{` `|` `}` `~` and every character outside ASCII with `unknown key`, and types nothing of a call that holds one. Under macOS's raw mode, which leaves a terminal's extended input on, `ctrl+o` and `ctrl+v` reach no program, the terminal taking them; with it off, Rex types `^O` and `^V`.

**boomdrive-design's first ruling**, on the construct's stop-short: "1. Not as you recommend: a bare character send-key refuses (your nineteen and every character outside ASCII) goes in as the text it is, by send, since a person typing it bare types exactly that text and 222 refuses only a key the terminal cannot type; Rex can type it, by send, as your send -- é showed. Only such a character led by ctrl+, alt+ or shift+ is refused, named as a key Rex 0.1.0-preview.14 cannot type, nothing sent for it, the keys around it still going in, in order. Refusing a bare ? @ _ ( would leave a relayed person unable to type a question or a path. 2. Overruled: give the block's last rows lines, by block call size, as the screen row does; 222 gives what the terminal shows as a person's view would, which is the screen, and herdr's row reads --source visible. The whole scrollback at four reads a second is not that. Keep the colors by the per-line reset, as you read it." Not taken: each such character refused, bare or led, the construct's recommendation; the whole capture, the row as it stood at `3d0200b`.

**boomdrive-design's second ruling**, on the code stage's found that Rex types a capital as its small letter: "(a), with one more: [...] A capital joins the nineteen and the characters outside ASCII: bare, it goes in as the text it is by send; led by ctrl+, alt+ or shift+, it is refused, named as a key Rex 0.1.0-preview.14 cannot type, nothing sent for it, the keys around it still going in, in order. ctrl+shift+ and a letter, which you read typing nothing, is refused the same way, since 222 never lets a key go silently untyped. No escape sequence is built. The rule under all of it: a key handed to send-key must type as the person typed it on your reading of the pinned Rex; any key you read that does not is a bare character sent as text, or refused where led." Not taken: a bare capital sent to `send-key` as `shift+` its small letter, and a led one as Rex takes it.

So a key goes to `send-key` only where the reading above has Rex type it as typed. Bare, a capital, one of the nineteen or a character outside ASCII goes in as the text it is by `send`, each run of them one `send`. Led, it is refused, named as a key Rex 0.1.0-preview.14 cannot type, and so is each led key of the list above, with nothing sent and no escape sequence built for it; the keys around it go in, in order, and the input is given back as untyped naming each, as herdr's relay gives back its six.

The construction agents' own choices, each with the path not taken:

- the ask makes no call, not a `process` call first;
- the capture is taken as written, not read as JSON;
- the cut keeps Rex's line endings, not the screen row's `\n`;
- `size` is read at each read, not once at the ask, which would miss a resize;
- each run of keys is one `send-key`, and each run of bare characters sent as text one `send`, not one call a key;
- the calls are bounded at one second, not the profile's ten;
- `send`'s text is named `<typed>` in flume's words for every `send`, a prompt's pieces as well as a relay's, not for the relay's alone;
- the characters and keys Rex does not type as typed are lists the profile holds, not learned from Rex's refusal of a whole call;
- a key types as typed where it types what a terminal's modifier forms give it, as Rex gives every named key, `ctrl+0` typing `0` and `shift+space` a space among them, not only where its bytes differ under each of its leads;
- the tools layer reads every key name alone, the relay's typing and the pinned Rex's own `send-key` of each key the relay does not hand it, not a sample of them.

### Consequences

- Rule core.57's details say how Rex relays: Rex leaves the terminals that offer no relay, and the details gain its bound, its last rows and the keys it does not type as typed.
- Good: Boomdrive's runner relays a Rex place by the same `flume relay`, with no change of its own.
- Bad: under Rex 0.1.0-preview.14 a person's led capital, `ctrl+shift+` and a character, `ctrl+` and one of the twelve, and `alt+shift+` and a character its shift changes do not reach the agent. The caller is told which key did not go in, and a Rex that comes to type one of them as typed shows in the tools layer, which reads the pinned Rex's own `send-key` of each.
- The row at `bf9e16f` names, of the led keys Rex types otherwise than typed, `ctrl+shift+` and a letter; the reading above finds the rest under the rule boomdrive-design gave, and the row is the design's to bring to it.
