# Design: flume hears sluice by subscription

Record 0082 holds the decision. This is how it is built.

## 1. The resident's loop

`flume host` builds a runtime of one thread and runs
`resident::run`. The loop settles, then waits for the first of three
things: its stop, a change heard, or a minute after that settle. A
change heard is followed by a settle two seconds later, so the writes
sluice makes as it decides one submission make one settle. A
`tokio::sync::Notify` holds one permit, so the changes heard while a
settle runs make one settle after it (one-read-in-flight). The minute
runs from the last settle, whatever started it, so the backstop settles
only where no change was heard within the minute.

## 2. The settle

Each settle builds the core afresh through `profiles::core`, so it
reads the host's configuration as it stands (core.45), and runs on
tokio's blocking pool: `settle_every` with a bound of a minute, then
`followed`. The core stays synchronous
(a-residents-core-stays-synchronous-behind-a-tokio-edge). The watch it
writes holds while the settle runs and two minutes after it ends, so
flume runs attended while the resident runs (208).

## 3. What is followed

`Engine::followed` reads held state alone, as the pending read does: the
repository of each live place whose submission is open, and of each
base place, whose base may move (124, 203, 225). No seam is asked.

## 4. The subscription

The resident spawns one task, tracked by a `TaskTracker`, that holds the
subscription the profiles give, `Subscribes::hold`, and holds it again a
minute after it ends. The set to follow reaches it through a
`tokio::sync::watch` channel, whose reader takes the newest set.

The sluice profile's `Subscription` waits until there is something to
follow, then connects to sluice host's socket: `host-<key>.sock` in the
user's own directory, the key the first 10 hex digits of the SHA-1 of
sluice's resolved state folder, as sluice names it. It says
`{"hello":"flume/<version>"}`, reads the answer, asks
`{"ask":"subscribe"}`, and sends `add` and `remove` lines for each
repository's folder, the one the profile's commands run in. Of each
`document` line it reads each branch's name, `version`, `revision` and
`gone`: sluice moves `version` on every write of a branch's records. A
reading that differs from the folder's last is a change; the first, sent
as the folder is added, is not. An `error` line or an unreadable line
ends the subscription with its words. No socket answering ends it too;
flume starts no sluice host (the-service-manager-starts-each-resident).

## 5. One resident on a host

`flume host` takes an exclusive `flock` on `host.lock` in flume's state
folder before anything else, and is refused while another holds it.

## 6. Stopping

SIGTERM and SIGINT cancel the token. The loop leaves its wait, the
subscription's task ends with the token, and the resident ends once the
tracker is empty. The subscription is a socket, so no child process is
left (nothing-detached).

## 7. The service

nix-darwin: `launchd.user.agents.flume-host`, run as the primary user
with `/bin/wait4path /nix/store` first, `KeepAlive`, `RunAtLoad`, the
user's profiles first on `PATH` and its log in
`~/Library/Logs/flume-host.log`. NixOS: `systemd.user.services.flume-host`,
`ConditionUser` the module's user, `Restart = always`. On each, a
terminal's server a start brought up is the terminal's and outlives the
resident: launchd's `AbandonProcessGroup`, systemd's
`KillMode = process`. Neither is declared where `package` is null.
