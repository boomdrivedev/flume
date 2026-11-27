# The-gate spec's CI build of the flake's package (the change
# flume-publishes-its-own-flake-and-module, design 11), its conventional
# check's convco (the change ci-checks-commits-with-the-hooks-convco) and
# its steps that download from GitHub run through try-again.sh (the change
# ci-tries-a-download-github-turns-away-again) and its releases, one run
# at a time (the change releases-never-share-a-tag), held by the
# workflow's text: .github/workflows/ci.yml read line by line, as yamlfmt
# lays it out. GitHub runs it; the gate holds what it says.
t:
let
  inherit (t) lib;
  inherit (lib.lists) findFirstIndex;

  lines = lib.splitString "\n" (builtins.readFile ../../.github/workflows/ci.yml);

  # The lines after the line `head`, up to the next at its own indent or
  # less, less the empty ones.
  blockAfter =
    indent: head: from:
    let
      at = findFirstIndex (l: l == "${indent}${head}") null from;
      after = lib.drop (at + 1) from;
      end = findFirstIndex (
        l: l != "" && builtins.match "${indent} .*" l == null
      ) (builtins.length after) after;
    in
    if at == null then [ ] else builtins.filter (l: l != "") (lib.take end after);

  on = blockAfter "" "on:" lines;
  jobs = blockAfter "" "jobs:" lines;
  fast = blockAfter "  " "fast:" jobs;
  flake = blockAfter "  " "flake:" jobs;
  conventional = blockAfter "  " "conventional:" jobs;
  concurrency = blockAfter "" "concurrency:" lines;
  version = blockAfter "  " "version:" jobs;
  binaries = blockAfter "  " "binaries:" jobs;
  release = blockAfter "  " "release:" jobs;
  has = line: from: builtins.elem line from;
  hasInfix = part: lib.any (lib.hasInfix part);
in
{
  "The workflow's build job" =
    t.expect (flake != [ ]) "the workflow holds no job flake"
    ++ t.expect (has "        os: [ubuntu-latest, macos-latest]" flake) "the job flake runs on no ubuntu-latest and macos-latest"
    ++ t.expect (has "    runs-on: \${{ matrix.os }}" flake) "the job flake runs on no matrix.os"
    ++ t.expect (has "      fail-fast: false" flake) "the job flake's strategy names no fail-fast: false"
    ++ t.expect (has "      - run: sh try-again.sh devenv shell -- moon run nix:build" flake) "the job flake runs no devenv shell -- moon run nix:build through try-again.sh"
    ++ t.expect (
      has "  push:" on && has "    branches: [main]" on && has "  pull_request:" on
    ) "the workflow runs on no push to main and pull request: ${builtins.toJSON on}"
    ++ t.expect (!lib.any (lib.hasPrefix "    if:") flake) "the job flake runs on a condition";

  "The workflow's conventional check" =
    t.expect (conventional != [ ]) "the workflow holds no job conventional"
    ++ t.expect (
      hasInfix "jq -r '.nodes.nixpkgs.locked | " conventional && hasInfix "' devenv.lock)" conventional
    ) "the job conventional reads no nixpkgs devenv.lock pins"
    ++ t.expect (hasInfix "nix shell \"$nixpkgs#convco\" -c convco check --ignore-reverts " conventional) "the job conventional runs no convco check --ignore-reverts of the nixpkgs devenv.lock pins"
    ++ t.expect (
      !hasInfix " nixpkgs#convco" conventional
    ) "the job conventional runs the flake registry's convco";

  "The workflow's downloads from GitHub" =
    t.expect (has "      - run: sh try-again.sh devenv shell -- moon run ':#check-fast'" fast) "the job fast runs no devenv shell -- moon run ':#check-fast' through try-again.sh"
    ++ t.expect (lib.all
      (l: !lib.hasInfix "devenv shell --" l || lib.hasInfix "sh try-again.sh devenv shell --" l)
      (fast ++ flake)
    ) "a step of the job fast or flake enters the devenv shell other than through try-again.sh"
    ++ t.expect (hasInfix "sh try-again.sh nix shell \"$nixpkgs#convco\" -c convco check " conventional) "the job conventional runs no nix shell of convco through try-again.sh"
    ++ t.expect (hasInfix "sh try-again.sh cargo build --release --locked " binaries) "the job binaries runs no cargo build through try-again.sh";

  "The releases one at a time" =
    t.expect (has "  group: \${{ github.event_name == 'push' && 'release' || format('pull-request-{0}', github.run_id) }}" concurrency) "the workflow puts the runs of pushes in no one group release: ${builtins.toJSON concurrency}"
    ++ t.expect (has "  cancel-in-progress: false" concurrency) "the workflow's group cancels a run in progress: ${builtins.toJSON concurrency}"
    ++ t.expect (
      !lib.any (lib.hasPrefix "    concurrency:") jobs
    ) "a job names a group of its own, which waits on its own run"
    ++ t.expect (
      hasInfix "jq -r '.nodes.nixpkgs.locked | " version && hasInfix "' devenv.lock)" version
    ) "the job version reads no nixpkgs devenv.lock pins"
    ++ t.expect (has "          tag=\"$(nix shell \"$nixpkgs#git-cliff\" -c sh next-version.sh)\"" version) "the job version runs no sh next-version.sh under the git-cliff of the nixpkgs devenv.lock pins"
    ++ t.expect (hasInfix "nix shell \"$nixpkgs#git-cliff\" -c " release) "the job release runs no git-cliff of the nixpkgs devenv.lock pins"
    ++ t.expect (
      !hasInfix " nixpkgs#git-cliff" (version ++ release)
    ) "the workflow runs the flake registry's git-cliff";
}
