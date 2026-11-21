#!/bin/sh
# shellcheck disable=SC2016 # each stand-in's script names its own $n.
# The cases of try-again.sh, which root:try-again runs: each runs
# sh try-again.sh over a stand-in command in a folder of its own, with a
# stand-in sleep first on PATH that records each pause and waits for none,
# and reads its exit, its words, how many times the stand-in ran and the
# pauses. A stand-in prints, at its nth run, what its case gives that run.
# It prints each failure as one line, try-again-cases: <case>: <what>,
# exits non-zero where any case fails, and says every case holds
# otherwise.
set -eu

here=$(cd "$(dirname "$0")" && pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0
failed=0

mkdir "$scratch/bin"
cat >"$scratch/bin/sleep" <<'EOF'
#!/bin/sh
echo "$1" >>pauses
EOF
chmod +x "$scratch/bin/sleep"

# try-again.sh run over `sh -c "$1"` in the case's own folder, where the
# stand-in's runs so far are `$n`: its exit, its words, the runs and the
# pauses.
try() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  mkdir "$dir"
  status=0
  (cd "$dir" && PATH="$scratch/bin:$PATH" sh "$here/try-again.sh" sh -c \
    'n=$(($(cat runs 2>/dev/null || echo 0) + 1)); echo "$n" >runs; '"$1") \
    >"$dir.out" 2>&1 </dev/null || status=$?
  out=$(cat "$dir.out")
  runs=$(cat "$dir/runs" 2>/dev/null || echo 0)
  pauses=$(cat "$dir/pauses" 2>/dev/null | tr '\n' ' ' || true)
}

# A failure of the case `$1`, saying `$2`, where the test after them fails.
expect() {
  named=$1
  what=$2
  shift 2
  if ! "$@"; then
    echo "try-again-cases: $named: $what"
    failed=1
  fi
}

# `$1` holds `$2`.
says() {
  case $1 in *"$2"*) ;; *) return 1 ;; esac
}

named="A command that passes"
try 'echo built'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 0 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 1 ]
expect "$named" "it paused: $pauses" [ -z "$pauses" ]
expect "$named" "its words are not the command's: $out" says "$out" "built"

# nix's words when GitHub turned a tarball away, its 429 coloured as nix
# colours it, and GitHub's own words under them.
named="A download GitHub turned away once"
try 'if [ "$n" -eq 1 ]; then
  printf "warning: unable to download %s: HTTP error \033[35;1m429\033[0m\n" "https://api.github.com/repos/NixOS/nixpkgs/tarball/f9a75f4"
  echo "429: This request was rate-limited due to too many requests from your network."
  exit 1
fi
echo built'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 0 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 2 ]
expect "$named" "it paused not a minute: $pauses" [ "$pauses" = "60 " ]
expect "$named" "it says not that it runs again: $out" \
  says "$out" "was turned away at run 1 of 3"
expect "$named" "its words are not the second run's: $out" says "$out" "built"

# GitHub's words for its rate limit, given with a 403, at every run.
named="A download GitHub turned away at every run"
try 'echo "error: unable to download https://api.github.com/repos/boomdrivedev/weir/tarball/cc9d41f: HTTP error 403"
echo "API rate limit exceeded for 203.0.113.7."
exit 7'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 7 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 3 ]
expect "$named" "it paused not a minute and then two: $pauses" \
  [ "$pauses" = "60 120 " ]
expect "$named" "it says not that it was turned away at each run: $out" \
  says "$out" "turned away at each of 3 runs"

# cargo's words where GitHub's git host answered 503, and then git's.
named="A crate's fetch turned away, then the git host's"
try 'case $n in
1) echo "error: failed to get successful HTTP response from https://github.com/boomdrivedev/weir, got 503"; exit 101 ;;
2) echo "fatal: unable to access https://github.com/boomdrivedev/weir/: The requested URL returned error: 502"; exit 101 ;;
esac
echo built'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 0 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 3 ]
expect "$named" "it paused not a minute and then two: $pauses" \
  [ "$pauses" = "60 120 " ]

named="A check that fails"
try 'echo "test a_case ... FAILED"; exit 3'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 3 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 1 ]
expect "$named" "it paused: $pauses" [ -z "$pauses" ]

named="A download GitHub denied"
try 'echo "error: unable to download https://api.github.com/repos/boomdrivedev/private/tarball/0a1b2c3: HTTP error 403"; exit 1'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 1 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 1 ]
expect "$named" "it paused: $pauses" [ -z "$pauses" ]

# A refusal the tool got past, its retry's words among the first lines,
# and forty lines after it before the command failed on a check.
named="A refusal the tool got past before a check failed"
try 'echo "warning: unable to download https://api.github.com/repos/NixOS/nixpkgs/tarball/f9a75f4: HTTP error 429; retrying in 3613 ms (attempt 1/5)"
i=0
while [ "$i" -lt 40 ]; do echo "test case_$i ... ok"; i=$((i + 1)); done
echo "test a_case ... FAILED"
exit 3'
expect "$named" "try-again.sh exits $status: $out" [ "$status" -eq 3 ]
expect "$named" "the command ran $runs times" [ "$runs" -eq 1 ]
expect "$named" "it paused: $pauses" [ -z "$pauses" ]

if [ "$failed" -ne 0 ]; then
  exit 1
fi
echo "try-again-cases: every case holds"
