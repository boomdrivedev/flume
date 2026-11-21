#!/bin/sh
# A step of CI that downloads from GitHub (.github/workflows/ci.yml), run
# again where GitHub turned a download away for a moment: the command
# runs, its output reaching the step's log as it comes, its standard error
# with its standard output; where it fails and the last lines of its
# output say HTTP 429 or a 5xx, or GitHub's rate limit, it runs again
# after a pause, up to three runs in all, a minute's pause before the
# second and two before the third, as GitHub asks a client it turned away
# to wait at least a minute. Any other failure, and a third refusal, exits
# with the command's own status at once. Only the end of the output is
# read: the tools retry a refusal themselves first, and a refusal they
# got past, earlier in the output, is not why the command failed.
set -u

tries=3
pause=60
refused='(HTTP error|returned error:|got|status code:?) (429|5[0-9][0-9])([^0-9]|$)|rate limit exceeded|secondary rate limit|rate-limited|Too Many Requests'
esc=$(printf '\033')
log=$(mktemp)
trap 'rm -f "$log" "$log.status"' EXIT

run=1
while :; do
  {
    "$@" 2>&1
    echo "$?" >"$log.status"
  } | tee "$log"
  status=$(cat "$log.status")
  if [ "$status" -eq 0 ]; then
    exit 0
  fi
  # The last lines, less the colours a tool writes, read for a refusal.
  words=$(tail -n 40 "$log" | sed "s/$esc\[[0-9;]*m//g" | grep -E -m 1 "$refused") ||
    exit "$status"
  if [ "$run" -ge "$tries" ]; then
    echo "try-again.sh: turned away at each of $tries runs, the last saying: $words"
    exit "$status"
  fi
  echo "::warning::try-again.sh: $1 was turned away at run $run of $tries, saying: $words; it runs again in ${pause}s"
  sleep "$pause"
  run=$((run + 1))
  pause=$((pause * 2))
done
