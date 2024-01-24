{
  description = "Minimal, blazing fast Node.js script runner";

  nixConfig = {
    extra-substituters = ["https://cache.garnix.io"];
    extra-trusted-public-keys = ["cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

    inherit (nixpkgs) lib;

    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: lib.genAttrs systems (s: fn nixpkgs.legacyPackages.${s});
  in {
    checks = forAllSystems (pkgs: let
      formatter = self.formatter.${pkgs.system};
    in {
      fmt =
        pkgs.runCommand "check-fmt" {}
        ''
          ${pkgs.lib.getExe formatter} --check ${self}
          touch $out
        '';
    });

    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rust-analyzer
          rustc
          cargo
          rustfmt
        ];

        RUST_BACKTRACE = 1;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });

    packages = forAllSystems (
      pkgs: let
        scope = lib.makeScope pkgs.newScope;
        fn = final: {p = self.overlays.default final pkgs;};
        inherit (scope fn) p;
      in
        p // {default = p.nrr;}
    );

    formatter = forAllSystems (p: p.alejandra);

    overlays.default = _: prev: {
      nrr = prev.callPackage ./default.nix {
        inherit self version;
        inherit (prev.darwin.apple_sdk_11_0.frameworks) CoreFoundation Security;
        inherit (prev.darwin) IOKit;
      };
    };
  };
}
