{
  lib,
  pkgsCross,
  self,
}:
let
  crossTargets = [
    pkgsCross.musl64.pkgsStatic
    pkgsCross.aarch64-multiplatform.pkgsStatic
  ];
in
builtins.listToAttrs (
  map (
    pkgs:
    let
      package = pkgs.callPackage ./package.nix { inherit self; };
    in
    lib.nameValuePair (builtins.parseDrvName package.name).name package
  ) crossTargets
)
