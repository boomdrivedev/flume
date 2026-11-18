# Tasks

The work is on `fix/main--sluice-pin-reads-the-hosts-config`, cut from
`main`. The statement is read at boomdrivedev/blueprints
`design/flume-confirms-sluices-outcomes` 250cc98 (C.6's inbox rows; 101,
112, 133); sluice's at v0.93.2 (sluice 51; sluice record 0102).

## 1. The pin

- [x] 1.1 Pin sluice 0.93.2 in `devenv.nix`, its three checksums those of boomdrivedev/sluice v0.93.2's assets, and the profile's module say it is checked against 0.93.2 (core.22). Verify with `sluice exec main -- moon run root:toolchain`.

## 2. The profile confirms what it collects

- [x] 2.1 Collect by `sluice inbox --confirm --json` (101, 133, C.6). Verify with `sluice exec main -- cargo test -p flume --features tools --test tools merge_queue_sluice`, "A failed entry collected once" and the merge-queue seam's scenarios.
- [x] 2.2 Confirm a submit's decided answer by `sluice confirm <key>` of its `delivery` where it is owed, and have `a_settle_beside_the_stand_ins_submit_collects_at_most_the_outcome_its_answer_gives` take the answer's outcome from an inbox beside the submit at most once (112, 133). Verify with the same suite, "A decided answer is confirmed".
- [x] 2.3 Verify the walkthroughs that send a place back, with `sluice exec main -- moon run flume:tools`.

## 3. The spec

- [x] 3.1 Validate with `openspec validate the-sluice-profile-confirms-what-it-collects --strict`, then archive with `openspec archive the-sluice-profile-confirms-what-it-collects --yes`.
