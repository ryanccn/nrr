{
  description = "Minimal, blazing fast npm scripts runner";

  nixConfig = {
    extra-substituters = [ "https://cache.garnix.io" ];
    extra-trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
  };

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (
        system:
        import nixpkgs {
          inherit system;
          config = { };
          overlays = [ self.overlays.default ];
        }
      );
    in
    {
      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};

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
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
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

            inputsFrom = [ self.packages.${system}.nrr ];

            __structuredAttrs = true;
            env = {
              RUST_BACKTRACE = 1;
              RUST_SRC_PATH = toString pkgs.rustPlatform.rustLibSrc;
            };
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          inherit (pkgs) nrr;
          default = pkgs.nrr;
        }
        // (lib.attrsets.mapAttrs' (
          name: value: lib.nameValuePair "check-${name}" value
        ) self.checks.${system})
      );

      legacyPackages = forAllSystems (system: nixpkgsFor.${system}.callPackage ./nix/static.nix { });

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      overlays.default = _: prev: { nrr = prev.callPackage ./nix/package.nix { }; };
    };
}
