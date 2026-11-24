## ADDED Requirements

### Requirement: flume host keeps watch over the host

`flume host`, flume's resident, SHALL settle the places where something
is pending as a watch's settle does, at once, again at each change its
merge queue's subscription hears, and again a minute after its last
settle where it heard none; changes heard together, and while a settle
runs, SHALL make one settle after it (208, 225; record 0082; core.13).

#### Scenario: A minute with nothing heard

- **WHEN** the resident runs over a paused clock and hears no change for four minutes
- **THEN** it settled at once and then once each minute

#### Scenario: A change heard

- **WHEN** the resident hears a change thirty seconds after its first settle
- **THEN** it settles two seconds later, settles nothing in the minute after that settle, and settles once that minute ends

#### Scenario: Many changes heard together

- **WHEN** twenty changes are heard at once and one more while the settle they began waits
- **THEN** they make two settles, and nothing more settles with nothing heard since

#### Scenario: A stop

- **WHEN** the resident is stopped
- **THEN** it ends within a second, its subscription with it

### Requirement: The resident follows the repositories its merge queue can move

The resident SHALL follow, on one subscription to the merge queue's
resident, the repository of each live place whose submission is in
flight and of each base place, read from held state alone, and no
other; it SHALL ask nothing while it follows none, and SHALL hold the
subscription again a minute after it ended, settling each minute
meanwhile (124, 203, 225; record 0082).

#### Scenario: A chore followed while in flight

- **WHEN** a chore works, is submitted and stays open, and then lands
- **THEN** no repository is followed while it works, its repository while its submission is open, and none once it landed and was tidied

#### Scenario: The newest set followed

- **WHEN** the resident's settles give one repository, then two, then none
- **THEN** its subscription follows each set in turn

#### Scenario: A subscription that ended

- **WHEN** the subscription ends a second after the resident starts
- **THEN** it is held again a minute later, and the minute's settle ran meanwhile

### Requirement: The resident hears sluice on sluice host's socket

The sluice profile's subscription SHALL connect to sluice host's socket,
say its hello, ask a subscription and add and remove each followed
repository's folder, starting no sluice command and no sluice host; a
document SHALL be a change only where a branch's records' version,
revision or gone moved since the folder's last, and an error line or no
socket SHALL end it with words (record 0082).

#### Scenario: Followed, heard and ended

- **WHEN** a stand-in sluice host answers on a socket, sends a folder's first document, the same reading again, a document of a folder not followed, then a document whose branch's version moved, then the followed set changes, then it sends an error line
- **THEN** flume said its hello, asked the subscription and added the folder, heard a change only at the moved version, removed the old folder and added the new, and the subscription ended naming the error's words

#### Scenario: Nothing to follow, then no sluice host

- **WHEN** nothing is followed, then one repository is, and nothing answers on the socket
- **THEN** nothing is asked while nothing is followed, and the subscription ends naming the socket

#### Scenario: The socket is sluice's

- **WHEN** sluice's state folder is under a given `XDG_STATE_HOME` and `SLUICE_USER_SCRATCH` names the user's directory
- **THEN** the socket is `host-<key>.sock` there, the key the first 10 hex digits of the SHA-1 of the resolved state folder
