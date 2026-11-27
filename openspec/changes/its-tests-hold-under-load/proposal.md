# Its tests hold under load

## Why

Main's gate fails submissions on tests that pass alone. Each landing
then waits on the next fix. We ran `flume:tools` and `flume:test` on
main many times, two copies of each at once on a busy host. These
tests failed at least once, and each passes alone:

- `open_sessions::s4_an_answer_within_the_hour_by_a_retry_and_a_message_reaches_the_same_session`
  failed in each copy of each round. The agent took its prompt and
  stopped short before a settle read the prompt received. The stage
  ended short with no receipt. The deliver part's take answers nothing
  while a prompt has no receipt, so the agent never took the answer.
  The retry waited its ten minutes and was refused.
- `walkthroughs::a_lost_talk_whose_transcript_is_gone_starts_afresh_with_flumes_words`:
  one settle read talk's prompt received. A second settle ended the
  start lost before the first wrote. The first then wrote the stage
  begun over the lost end, with no live agent.
- `terminal_rex::a_text_past_sixty_thousand_bytes_arrives_whole`
  failed in each copy of most rounds. The test's stand-in read its
  terminal one kilobyte at a time. It split each line with shell
  expansions whose cost grows with the square of the text. 150,000
  bytes took more than 60 seconds on the busy host. The stand-in also
  reported idle before it set its terminal, so a text typed at once
  could lose its Enter.
- `open_sessions::s104_the_time_passing_lets_the_agent_go_and_a_late_answer_starts_a_fresh_one`:
  the host keeps the agent open five seconds. The test had to read the
  agent and ask for a second fix in those five seconds.
- `window_ghostty::ghostty_on_macos_passes_the_window_scenarios` and
  `window_ghostty::the_words_open_is_given`: the tests held each show
  to two seconds. A shell stand-in took longer to start on the busy
  host. The profile's own bound is ten seconds.
- `walkthroughs::a_pull_requests_failure_goes_back_to_code_once_and_its_next_request_lands`
  also failed. Another change fixes its cause, and this change does
  not touch it.

## What Changes

- **A stop-short reads its prompt's receipt.** When an agent stops
  short and no settle recorded its prompt received, the core reads the
  receipt from the agent. It records the receipt with the end. The
  deliver part then takes the answer that the agent is kept open for.
- **A receipt never writes over a newer end.** A settle records a
  receipt, or the word that an agent started, only while the start is
  live and its stage still starts. Another settle's end stays.
- **Rex's stand-in reads a long text in linear time.** It sets its
  terminal before it reports idle. One `cat` drains the terminal into a
  pipe that the loop reads 64 KiB at a time. One field split cuts a
  line at its Enters. The long-text test makes the stand-in read three
  seconds after it reports idle.
- **s104 orders its steps.** Code stops short only after the test read
  its agent and asked for the second fix.
- **The window profiles' tests use the profile's own bound.** Only the
  scenario of a program that never answers uses two seconds.

Each fix in the core has a conformance scenario that forces its race.
That scenario fails without the fix.
