# Design: flume status reads flume host

The Boom decision `flume-host-serves-status-on-its-socket` holds the
decision. This page says how it is built.

## 1. The socket

`flume host` listens on one Unix socket, as sluice host does. The socket
is `host-<key>.sock` in the user's own directory. The user's directory
is `/tmp`, resolved, then `flume-<uid>`, or `$FLUME_USER_SCRATCH` where
it is set. The key is the first 10 hex digits of the SHA-1 of flume's
resolved state folder, `$XDG_STATE_HOME/flume` or `~/.local/state/flume`.
The directory has mode 0700. `flume host` removes a socket that no
process answers on before it listens, and removes its own socket when it
stops. It takes its lock, `host.lock`, before it touches the socket, so a
second `flume host` never removes the socket of the first.

## 2. The lines

Each line is one JSON object. The protocol is the residents' protocol
(residents-speak-json-lines-over-a-unix-socket).

The client says its hello first:

```json
{"hello":"boomdrive/0.40.0"}
```

`flume host` answers with its version, its protocol, its process and
when it started:

```json
{"version":"0.36.0","protocol":1,"pid":4242,"since":"2026-11-24T18:00:00.000Z"}
```

The client then asks one thing on the connection.

**Status.** One answer, then `flume host` closes the connection.

```json
{"ask":"status"}
{"ask":"status","place":"/Users/me/.local/share/flume/places/flume/fix--one"}
{"ask":"status","line":true}
```

`place` is a place's location, as status gives it. With no `place`, the
answer covers every place. `line` asks each place's one line, as
`flume status --line` does. The answer is one line:

```json
{"document":{"status":{"places":[...],"host":{...}}}}
```

The value of `document` is the answer exactly as `flume --json status`
prints it. A refusal is `{"document":{"refused":{...}}}`, as
`flume --json status` prints one.

**Subscribe.** The connection stays open.

```json
{"ask":"subscribe"}
```

It takes `place` and `line` as `status` does. `flume host` sends a
`document` line at once. It sends a new `document` line each time the
answer differs from the last one it sent on that connection. It sends
nothing when the answer is the same. The client ends the subscription
when it closes the connection.

**Errors.** A line that `flume host` cannot read, or an ask that it does
not know, gets one line, and then `flume host` closes the connection:

```json
{"error":"flume host cannot read the line {\"ask\":\"x\"}: unknown variant `x`"}
```

## 3. When flume host reads status again

`flume host` reads status again for its subscriptions:

- after each settle it makes,
- 2 seconds after a file event in flume's state folder, where each
  command of flume writes the state of its places, the events of those
  2 seconds together making one read, and
- after each change that its subscription to sluice host hears.

Many triggers make one read in flight (one-read-in-flight). A read runs
on the blocking pool of tokio, as a settle does. The core stays
synchronous (a-residents-core-stays-synchronous-behind-a-tokio-edge).
Each subscription gets the same read when it asks the same thing.

## 4. The submissions that flume host keeps

flume host keeps the newest records of each base, for each repository
that it reads, with the `version` of the records. sluice moves `version`
at each write of the records of a branch. The documents of the
subscription give `version` for each followed repository. When status
asks for the submissions of a repository, flume host asks sluice host's
socket `overview`, where the subscription does not follow the repository,
and `status <base> --all` for each base whose `version` moved. It answers
the rest from the records that it keeps. It starts no sluice command
(residents-ask-each-other-by-socket).

The asks are the asks of sluice host's protocol:

```json
{"hello":"flume/0.36.0"}
{"ask":"overview","folder":"/repo/.git","json":true}
{"ask":"status","folder":"/repo/.git","target":"main","all":true,"json":true}
```

sluice host answers each with `{"output":{"stdout":"...","code":0}}`.
The profile reads `stdout` as it reads the output of the command today.
sluice host answers these asks from release 0.104.1. The tree pins
sluice 0.105.1.

## 5. The client

`flume status`, with or without `--json` and `--line`, finds the place
that it names as it does today. Then it asks flume host on its socket,
and prints the `document` as it printed the core's answer. It builds no
core and starts no process. Where no `flume host` answers on the socket,
`flume status` is refused. Its words name the socket and say that the
host's service manager runs `flume host`
(the-service-manager-starts-each-resident).

`flume host --no-watch` serves the socket and keeps its submissions, but
makes no settle and writes no watch. It is for a host where a caller
keeps watch in its stead, and for the tests, which settle by their own
commands.

## 6. Stopping

SIGTERM and SIGINT cancel the token of `flume host`. The listener, each
connection and each subscription end with it, and the socket file is
removed. No child process is left (nothing-detached).
