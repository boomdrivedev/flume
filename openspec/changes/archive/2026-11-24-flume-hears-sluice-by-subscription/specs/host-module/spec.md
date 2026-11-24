## RENAMED Requirements

- FROM: `### Requirement: The file is one user's, and flume runs no service`
- TO: `### Requirement: The file is one user's, and flume host is that user's service`

## MODIFIED Requirements

### Requirement: The file is one user's, and flume host is that user's service

On nix-darwin the module SHALL write the primary user's file, requiring
a primary user whose home is set. On NixOS it SHALL write the file of
`services.flume.user`, a user `users.users` declares, and do nothing in
any other user's activation. Where it installs a package, each SHALL
declare `flume-host` for that user alone, running the package's `flume
host`, kept running (record 0082; core.13).

#### Scenario: Enabled on nix-darwin

- **WHEN** `nix:test` evaluates a nix-darwin system of aarch64-darwin whose primary user is alice, at `/Users/alice`, with `services.flume.enable` set
- **THEN** the system profile holds the flake's aarch64-darwin `flume`, the post-activation runs the link as alice under `/Users/alice` with the file rendered from `settings`, `system.requiresPrimaryUser` names `services.flume.enable`, and the launchd agent `flume-host` alone is declared, running the flake's `flume host` under `/Users/alice`, started at load and kept running, alice's profile first on its `PATH`

#### Scenario: Enabled on NixOS

- **WHEN** `nix:test` evaluates a NixOS system of x86_64-linux with `services.flume.enable` set and `services.flume.user` alice, a normal user, beside the normal user bob
- **THEN** the system profile holds the flake's x86_64-linux `flume`, the user activation runs the link with the file rendered from `settings` where the user is alice and does nothing for bob, and the user unit `flume-host` alone is declared for flume, running the flake's `flume host` for alice alone, started with her manager and restarted always

#### Scenario: No user to write for

- **WHEN** `nix:test` evaluates a nix-darwin system whose primary user's home is unset, or a NixOS system whose `services.flume.user` no `users.users` entry declares
- **THEN** evaluation fails with an assertion naming the user and what to set

#### Scenario: No package, no service

- **WHEN** `nix:test` evaluates a nix-darwin system, and a NixOS system, each with `services.flume.package` null
- **THEN** neither declares a launchd agent or a unit for flume
