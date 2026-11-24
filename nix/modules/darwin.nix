# The nix-darwin module, darwinModules.flume (the change
# flume-publishes-its-own-flake-and-module, design 5, 9 and 10): flume in
# the system profile, and its managed file, rendered from `settings` at
# evaluation (design 7), linked at ~/.config/flume/config.toml for
# nix-darwin's primary user, since nix-darwin activates as root; after it,
# each source `sources` declares, rendered the same way, linked at
# ~/.config/flume/sources/<source>.toml, and the module's link of a source
# no longer declared removed (the change
# flumes-module-writes-the-hosts-sources, design 5 and 7). Where it
# installs a package, it declares the launchd agent `flume-host`, which
# runs that package's `flume host`, flume's resident, as the primary user,
# started at load and again whenever it ends (record 0082; core.13); a
# command reads the new files at once, and the resident at its next
# settle. The activation runs each step as the primary user under their
# own home, as Boomdrive's does, and a step that fails says why and stops
# nothing else.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.flume;
  toml = import ./toml.nix { inherit lib; };
  file = pkgs.writeText "flume-config.toml" (toml.text cfg.settings);
  # Each source the host declares, by the order of the sources' names, and
  # its file, rendered under its own option and named as the module's in
  # the store (the change flumes-module-writes-the-hosts-sources, design 3
  # and 5).
  sourceFile =
    name:
    pkgs.writeText "flume-source-${name}.toml" (
      toml.render "services.flume.sources.${name}" cfg.sources.${name}
    );
  sourceArgs = lib.concatMapStrings (
    name: " ${lib.escapeShellArg name} ${lib.escapeShellArg (sourceFile name)}"
  ) (builtins.attrNames cfg.sources);

  user = config.system.primaryUser;
  home = if user == null then null else config.users.users.${user}.home or null;
  # The user's own profiles first, then the system's, as a login shell
  # finds the tools the profiles run: git, the merge queue and the
  # terminal.
  path = lib.concatStringsSep ":" [
    "/etc/profiles/per-user/${toString user}/bin"
    "${toString home}/.nix-profile/bin"
    "${toString home}/.local/state/nix/profile/bin"
    "/run/current-system/sw/bin"
    "/nix/var/nix/profiles/default/bin"
    "/usr/local/bin"
    "/usr/bin"
    "/bin"
    "/usr/sbin"
    "/sbin"
  ];
in
{
  imports = [ (import ./options.nix { inherit packages; }) ];

  config = lib.mkIf cfg.enable {
    system.requiresPrimaryUser = [ "services.flume.enable" ];

    assertions = [
      {
        assertion = user == null || home != null;
        message = "services.flume writes the managed file of nix-darwin's primary user ${toString user}, whose home users.users.${toString user}.home is not set: set it, the home whose ~/.config/flume/config.toml the module links.";
      }
    ];

    environment.systemPackages = lib.optional (cfg.package != null) cfg.package;

    # flume's resident (record 0082): one `flume host` for the primary
    # user, kept running. A terminal's server one of its starts brought up
    # is the terminal's, and outlives it (core.13's details), so launchd
    # leaves its process group standing when it ends.
    launchd.user.agents.flume-host = lib.mkIf (cfg.package != null) {
      serviceConfig = {
        ProgramArguments = [
          "/bin/sh"
          "-c"
          "/bin/wait4path /nix/store && exec ${lib.getExe' cfg.package "flume"} host"
        ];
        EnvironmentVariables = {
          HOME = toString home;
          PATH = path;
        };
        KeepAlive = true;
        RunAtLoad = true;
        ProcessType = "Background";
        AbandonProcessGroup = true;
        ThrottleInterval = 30;
        StandardOutPath = "${toString home}/Library/Logs/flume-host.log";
        StandardErrorPath = "${toString home}/Library/Logs/flume-host.log";
      };
    };

    system.activationScripts.postActivation.text = ''
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
        /bin/sh ${./activate.sh} link ${file} || true
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
        /bin/sh ${./activate.sh} sources${sourceArgs} || true
    '';
  };
}
