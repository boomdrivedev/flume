## ADDED Requirements

### Requirement: flume host listens on its socket

`flume host` SHALL listen on `host-<key>.sock` in the user's own
directory, `flume-<uid>` in `/tmp` resolved or `$FLUME_USER_SCRATCH`,
the key the first 10 hex digits of the SHA-1 of flume's resolved state
folder, and SHALL answer a client's hello with its version, its
protocol, its process and when it started
(flume-host-serves-status-and-its-subscription-on-its-socket;
residents-speak-json-lines-over-a-unix-socket).

#### Scenario: The socket's name

- **WHEN** flume's state folder is under a given `XDG_STATE_HOME` and `FLUME_USER_SCRATCH` names the user's directory
- **THEN** the socket is `host-<key>.sock` there, the key the first 10 hex digits of the SHA-1 of the resolved state folder

### Requirement: flume host serves status and a subscription to it

After the hello, `flume host` SHALL answer `status` with one `document`
line, the answer `flume --json status` prints, and close the
connection; and `subscribe` with a `document` at once and another each
time the answer differs from the last one sent, until the client closes
it. Either ask MAY name a place and ask each place's line. A line it
cannot read SHALL get one `error` line, and the connection SHALL close
(flume-host-serves-status-and-its-subscription-on-its-socket).

#### Scenario: A status

- **WHEN** a client says its hello and asks `{"ask":"status","line":true}`
- **THEN** flume host answers its version, protocol and pid, then one `document` holding the answer, and closes the connection

#### Scenario: A subscription

- **WHEN** a client subscribes, the answer stays the same at one change, and then differs at five changes made together
- **THEN** the client gets the first document, nothing at the change with the same answer, and one document for the five changes

#### Scenario: An ask it does not know

- **WHEN** a client asks `{"ask":"settle"}`
- **THEN** flume host answers one `error` line naming the line, and closes the connection

### Requirement: flume host reads status again at each change

`flume host` SHALL read the status of each subscription again after
each of its settles, after each change its merge queue's subscription
hears, and two seconds after a file event in flume's state folder that
is not its log or a lock, the events of those two seconds making one
read (residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop;
one-read-in-flight).

#### Scenario: Read again after a settle

- **WHEN** the resident settles while a client subscribes
- **THEN** the subscription's status is read again, and sent where it changed
