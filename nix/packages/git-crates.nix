# flume's git crates: each git package of Cargo.lock, by
# `<name>-<version>`, to the revision its hash was taken at and the hash of
# fetchgit's tree there, which the package fetches it by (the change
# flume-publishes-its-own-flake-and-module, design 2). A revision moves with
# its hash, in one commit: nix:test fails where an entry's revision is not
# Cargo.lock's, or a git package has no entry. A hash is taken by building
# `fetchgit { url; rev; sha256 = lib.fakeHash; }` and reading the hash nix
# got.
{
  "weir-cadence-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-capture-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-delivery-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-engine-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-kinds-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-pack-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
  "weir-policy-0.0.0" = {
    rev = "cc9d41fba5e47b25ed1dde90c380d3c9f56a1785";
    hash = "sha256-KcK3LDwTs8gd72uK4lVNNyQyoVjdDM0lErVNMDY75Vk=";
  };
}
