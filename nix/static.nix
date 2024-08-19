{ lib, pkgsCross }:
let
  crossTargets = [
    pkgsCross.musl64.pkgsStatic
    pkgsCross.aarch64-multiplatform.pkgsStatic
  ];
in
builtins.listToAttrs (
  map (pkgs: lib.nameValuePair (builtins.parseDrvName pkgs.nrr.name).name pkgs.nrr) crossTargets
)
