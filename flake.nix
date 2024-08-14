{
  description = "Minimal, blazing fast npm scripts runner";

  nixConfig = {
    extra-substituters = [ "https://cache.garnix.io" ];
    extra-trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config = { };
          overlays = [
            inputs.rust-overlay.overlays.default
            self.overlays.default
          ];
        };

        inherit (pkgs) lib;

        mkFlakeCheck =
          {
            name,
            nativeBuildInputs ? [ ],
            command,
          }:
          pkgs.stdenv.mkDerivation {
            name = "check-${name}";
            inherit nativeBuildInputs;
            inherit (self.packages.${system}.nrr) src cargoDeps;

            buildPhase = ''
              ${command}
              touch "$out"
            '';

            doCheck = false;
            dontInstall = true;
            dontFixup = true;
          };
      in
      {
        checks = {
          nixfmt = mkFlakeCheck {
            name = "nixfmt";
            nativeBuildInputs = with pkgs; [ nixfmt-rfc-style ];
            command = "nixfmt --check .";
          };

          rustfmt = mkFlakeCheck {
            name = "rustfmt";
            nativeBuildInputs = with pkgs; [
              cargo
              rustfmt
            ];
            command = "cargo fmt --check";
          };

          clippy = mkFlakeCheck {
            name = "clippy";
            nativeBuildInputs = with pkgs; [
              rustPlatform.cargoSetupHook
              cargo
              rustc
              clippy
              clippy-sarif
              sarif-fmt
            ];
            command = ''
              cargo clippy --all-features --all-targets --tests \
                --offline --message-format=json \
                | clippy-sarif | tee $out | sarif-fmt
            '';
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer

            git-cliff # changelog generator
            taplo # TOML toolkit

            cargo-audit
            cargo-bloat
            cargo-expand

            libiconv
          ];

          __structuredAttrs = true;
          env = {
            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };
        };

        packages =
          {
            inherit (pkgs) nrr;
            default = pkgs.nrr;
          }
          // (lib.attrsets.mapAttrs' (
            name: value: lib.nameValuePair "check-${name}" value
          ) self.checks.${system});

        legacyPackages = import ./nix/static.nix pkgs;

        formatter = pkgs.nixfmt-rfc-style;
      }
    )
    // {
      overlays.default = _: prev: { nrr = prev.callPackage ./nix/package.nix { }; };
    };
}
