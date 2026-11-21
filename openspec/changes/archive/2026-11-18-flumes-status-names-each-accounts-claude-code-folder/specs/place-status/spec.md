## MODIFIED Requirements

### Requirement: Status gives the host's pools

Status SHALL give, for the host, each pool with its strategy and its
threshold, and each account with its agents running, its bound, the
time it stops being spent, the words in which no reset was read while
it is spent for the hour 146 gives, its reading and whether it is
fresh, or why it has none, and, while it is past its threshold, the
window that holds it and until when; and no value of an account's
environment and no path but its configuration home (56, 144 to 146,
184, 185, C.4; core.14).

#### Scenario: A pool with a spent account

- **WHEN** status is asked while work-1 runs two agents under a bound of four and work-2 is spent until 15:00
- **THEN** the pool `work` gives fill-first, work-1 with two running and a bound of four, and work-2 spent until 15:00 (56, 146)

#### Scenario: An account spent for an hour

- **WHEN** status is asked while work-3 is spent for an hour because no reset was read in "You've hit your weekly limit · resets Nov 8, 12am (UTC)"
- **THEN** work-3 gives its spent time and `spent_unread` with those words, and the person's form gives them under its line (56, 146)

#### Scenario: A pool with readings

- **WHEN** status is asked of a most-left pool stating no threshold, whose work-1 reads `five_hour` 42% used resetting at 17:00, taken two minutes ago from a fix's code agent in the session `s-1`, whose work-2's reading is twenty minutes old, and for whose work-3 no session has reported
- **THEN** the pool gives most-left and a threshold of 90, work-1 its window with 42, 17:00 and fresh, `read_at` when flume took it, the code agent's identity as `agent` and `s-1` as `session`, the JSON form keeping every field it gave before, work-2 its reading stale, and work-3 no reading with `no session flume started has reported its windows`, in the JSON form and the person's, and neither gives a path (56, 184, 185)

#### Scenario: An account past its threshold

- **WHEN** status is asked while work-1's reading gives `seven_day` 95% used, resetting Monday at 08:00
- **THEN** work-1 gives `past` with `seven_day` and Monday at 08:00, and its window `past` (56, 185)

## ADDED Requirements

### Requirement: Status gives each account's configuration home

Status SHALL give each account of a pool its configuration home: the
value of the variable its program's profile names, its `~` and each
`$VAR` read against the host user's login environment as a start reads
the account's `env`; and none where the profile names no variable, the
account's variables give none or give it empty. The JSON form SHALL
give `home` on every account, a path or `null` (56, 144, C.4; core.14).

#### Scenario: Each account's home

- **WHEN** the agent profile names `FAKE_HOME` as its home's variable, the login environment's `HOME` is `/home/op`, and status is asked of a pool whose work-1 gives `FAKE_HOME = "~/.agent-work-1"` and a token, work-2 gives no `FAKE_HOME`, work-3 gives it empty, and other-1, of a program whose profile names no variable, gives `FAKE_HOME = "~/.other"`
- **THEN** work-1 gives `home` `/home/op/.agent-work-1`, and work-2, work-3 and other-1 give `home` `null`, in the JSON form; the person's form gives `configuration home /home/op/.agent-work-1` under work-1's line and no home under the others; and neither form gives the token (56, C.4; core.30)

#### Scenario: The home among the fields the form holds

- **WHEN** the answers' suite reads a status whose pool holds an account with a home and one with `home_unread`
- **THEN** `status.host.pools[].accounts[].home` and `status.host.pools[].accounts[].home_unread` are among the fields it holds, so a form that drops either fails naming it (61; core.14)

### Requirement: Status says why it cannot read an account's home

Where the value of an account's home's variable names a variable the
host user's login environment lacks, status SHALL give the account no
home and `home_unread`, words naming the account's variable and the
one the login environment lacks and no value, in the JSON form and the
person's; and SHALL leave `home_unread` out of the JSON form while it
holds no words (56, C.4; core.14, core.30).

#### Scenario: A home the login environment cannot read

- **WHEN** status is asked of a pool whose work-4 gives `FAKE_HOME = "$WORK_HOMES/four"`, the agent profile naming `FAKE_HOME`, and the login environment gives no `WORK_HOMES`
- **THEN** work-4 gives `home` `null` and `home_unread` reading `env.FAKE_HOME names $WORK_HOMES, which the host user's login environment lacks`, the person's form gives `no configuration home:` with those words under its line, and no account that has a home or none gives `home_unread` (56, C.4)

### Requirement: An account's variables stay masked beside its home

Status SHALL give an account's configuration home with its account
among the host's pools alone, and SHALL give the variable that names
it as `<set>` wherever it gives an account's variables: in an agent's
settings and in the host's configuration values (56, C.4; core.30).

#### Scenario: The home given and its variable masked

- **WHEN** status is asked of a place whose code agent runs on work-1, whose `FAKE_HOME = "~/.agent-work-1"` names its home
- **THEN** the host's pools give work-1's home `/home/op/.agent-work-1`, and the agent's settings give the variable `FAKE_HOME` from `account` as `<set>`, and no other part of the answer gives its value as written or read (56; core.30)
