{
  lib,
  rustPlatform,
  ...
}: let
  toml = (lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage {
    pname = toml.name;
    inherit (toml) version;

    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;

    meta = {
      inherit (toml) description license;
      homepage = toml.repository;
      changelog = "${toml.repository}/blob/v${toml.version}/CHANGELOG.md";
    };
  }
