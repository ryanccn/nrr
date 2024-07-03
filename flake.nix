{
  description = "Minimal, blazing fast npm scripts runner";

  nixConfig = {
    extra-substituters = ["https://cache.garnix.io"];
    extra-trusted-public-keys = ["cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        config = {};
        overlays = [
          inputs.rust-overlay.overlays.default
          self.overlays.default
        ];
      };

      inherit (pkgs) lib;
    in rec {
      checks = {
        fmt = pkgs.runCommand "check-fmt" {} ''
          ${lib.getExe formatter} --check ${./.}
          touch $out
        '';
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          rust-analyzer
          rustc
          cargo
          rustfmt

          libiconv
        ];

        RUST_BACKTRACE = 1;
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      packages = {
        inherit (pkgs) nrr;
        default = pkgs.nrr;
      };

      legacyPackages = let
        staticPkgs = import ./nix/static.nix pkgs;
      in (lib.optionalAttrs pkgs.stdenv.isLinux staticPkgs);

      formatter = pkgs.alejandra;
    })
    // {
      overlays.default = _: prev: {
        nrr = prev.callPackage ./nix/package.nix {};
      };
    };
}
