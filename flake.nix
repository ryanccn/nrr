{
  description = "Minimal, blazing fast npm scripts runner";

  nixConfig = {
    extra-substituters = ["https://cache.garnix.io"];
    extra-trusted-public-keys = ["cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    inherit (nixpkgs) lib;

    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = fn: lib.genAttrs systems (s: fn nixpkgs.legacyPackages.${s});
  in {
    checks = forAllSystems ({
      lib,
      pkgs,
      system,
      ...
    }: {
      fmt =
        pkgs.runCommand "check-fmt" {}
        ''
          ${lib.getExe self.formatter.${system}} --check ${self}
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

    packages = forAllSystems ({
      pkgs,
      stdenv,
      system,
      ...
    }: let
      crane = inputs.crane.lib.${system};
      nrr = pkgs.callPackage ./nix/package.nix {
        inherit crane;
      };
    in
      {
        inherit nrr;
        default = nrr;
      }
      // lib.optionalAttrs stdenv.isLinux {
        nrr-static = pkgs.callPackage ./nix/static.nix {
          inherit crane nrr;
          fenix = inputs.fenix.packages.${system};
        };
      });

    formatter = forAllSystems (p: p.alejandra);

    overlays.default = _: prev: {
      nrr = prev.callPackage ./nix/package.nix {
        crane =
          inputs.crane.lib.${prev.stdenv.hostPlatform.system}
          or (prev.callPackage inputs.crane {});
      };
    };
  };
}
