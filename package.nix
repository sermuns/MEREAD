{
  pkgs,
  lib ? pkgs.lib,
  naersk,
  ...
}: let
  naersk' = pkgs.callPackage naersk {};
  toml = (lib.importTOML ./Cargo.toml).package;
in
  naersk'.buildPackage {
    pname = toml.name;
    inherit (toml) version;

    src = ./.;

    meta = {
      inherit (toml) description license;
      homepage = toml.repository;
      changelog = "${toml.repository}/blob/v${toml.version}/CHANGELOG.md";
    };
  }
