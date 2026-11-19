# Tasks

Each task names the narrowest check that shows it done. `<base>` is the
branch the work lands on. The statement is read at
boomdrivedev/blueprints main (4, 25, 49, 56, 223, 224, S130, C.2, C.5,
C.7). Design sections are this change's `design.md`.

## 1. The seams' types

- [x] 1.1 In `crates/flume-seams`, add `ChartStage.command`, `ChartSession.command`, `Launch.params`, `PlaceRecord.params`, `StartRecord.command`, `StageEnd.ran` with `Ran` and `Exited`, and `StageState::Failed`, each left out of JSON while none, and the paths `params_path`, `launched_word`, `ran_exit` and `ran_output` (223, 224, C.7; design 3, 5, 6). Check: `devenv shell -- cargo test -p flume-seams --lib round_trip`.
- [x] 1.2 Give the store seam `read_file`, the state directory's and the fake's, and add the store scenario "a file of the folder read back" (223, C.7; design 5). Check: `devenv shell -- cargo test -p flume-conformance --test seams`.
- [x] 1.3 Give the terminal seam `run`, refusing by default, and the host `launcher`, the real host's binary by its absolute path and the fake's own (223, C.4, C.5; core.60). Check: `devenv shell -- cargo test -p flume-seams --lib a_terminal_that_keeps_the_seams_default_runs_no_command`.

## 2. The command template and the launch

- [x] 2.1 Add `crates/flume-core/src/command@1.toml` and `atoms::templates()`, both templates' text, held by a new place; read every statechart kind of the templates' text in the weir kinds profile's and the fake kinds' `read_statechart` (175, 223, C.2; design 1, 2). Check: `devenv shell -- cargo test -p flume --test profiles kinds_weir`.
- [x] 2.2 Have the weir kinds profile read a command state's `params.command`, a list of strings (223, C.2). Check: `devenv shell -- cargo test -p flume --test profiles kinds_weir`.
- [x] 2.3 Have `chart_runs` take `command` beside `stage`, refuse a command stage flume cannot run, a send-back to a command stage and `{{params}}` with no parameters; keep a launch's parameters with the place and in its folder (4, 223, 224). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [x] 2.4 Give `flume launch` `--params-file` (224). Check: `devenv shell -- cargo test -p flume --lib`.

## 3. Running a command stage

- [x] 3.1 Record a command stage's one session with its command filled, take no words for it, and start it by `start_command` through the terminal's `run` (223, 224, C.5; design 3, 4). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [x] 3.2 Step a run from its launcher's word and recorded exit, deliver or fail it, read `stage.end` `passed` or `failed` under its bound, and give its failure to the next agent's stage (25, 112, 223; core.9; design 6, 7). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [x] 3.3 Have a retry naming a command stage run it again and refuse words, and a caller's stop end a run that works with no force (49, 51, 223). Check: `devenv shell -- cargo test -p flume-conformance --test engine`.
- [x] 3.4 Have flume's launcher run a command start's command as its child, keeping its output and exit, then settle the place (223, C.2; design 5). Check: `devenv shell -- cargo test -p flume --lib launcher`.
- [x] 3.5 Give the herdr, Rex and printed terminals `run` (223, C.5; design 8). Check: `devenv shell -- cargo test -p flume --lib`.

## 4. The rules, the suite and the site

- [x] 4.1 Amend core.9 and its details for a command stage's end (223; core.19). Check: `sluice exec <base> -- moon run root:format`.
- [x] 4.2 Name 223, 224 and S130 built in the clause list, cited by the command stages' scenarios (core.19). Check: `devenv shell -- cargo test -p flume-conformance`.
- [x] 4.3 Document command stages and `--params-file` in the kinds guide and the commands reference (223, 224; core.35). Check: `sluice exec <base> -- moon run site:build`.
