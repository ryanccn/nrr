{
  lib,
  pkgsCross,
  rust-bin,
  nrr,
  ...
}:
let
  targets = {
    x86_64 = pkgsCross.musl64.pkgsStatic;
    aarch64 = pkgsCross.aarch64-multiplatform.pkgsStatic;
  };

  toolchain = rust-bin.stable.latest.minimal.override {
    extensions = [ "rust-std" ];
    targets = map (pkgs: pkgs.stdenv.hostPlatform.config) (lib.attrValues targets);
  };

  rustPlatforms = lib.mapAttrs (lib.const (
    pkgs:
    pkgs.makeRustPlatform (
      lib.genAttrs [
        "cargo"
        "rustc"
      ] (lib.const toolchain)
    )
  )) targets;

  mkPackageWith =
    rustPlatform:
    nrr.override {
      inherit rustPlatform;
      lto = true;
    };
in
lib.mapAttrs' (
  target: rustPlatform: lib.nameValuePair "nrr-static-${target}" (mkPackageWith rustPlatform)
) rustPlatforms
