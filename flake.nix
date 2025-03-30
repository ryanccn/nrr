{
  description = "Minimal, blazing fast npm scripts runner";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
    in
    {
      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};

          mkFlakeCheck =
            {
              name,
              command,
              ...
            }@args:
            pkgs.stdenv.mkDerivation (
              {
                name = "check-${name}";
                inherit (self.packages.${system}.nrr) src;

                buildPhase = ''
                  ${command}
                  touch "$out"
                '';

                doCheck = false;
                dontInstall = true;
                dontFixup = true;
              }
              // (removeAttrs args [
                "name"
                "command"
              ])
            );
        in
        {
          nixfmt = mkFlakeCheck {
            name = "nixfmt";
            command = "find . -name '*.nix' -exec nixfmt --check {} +";

            src = self;
            nativeBuildInputs = with pkgs; [ nixfmt-rfc-style ];
          };

          rustfmt = mkFlakeCheck {
            name = "rustfmt";
            command = "cargo fmt --check";

            nativeBuildInputs = with pkgs; [
              cargo
              rustfmt
            ];
          };

          clippy = mkFlakeCheck {
            name = "clippy";
            command = ''
              cargo clippy --all-features --all-targets --tests \
                --offline --message-format=json \
                | clippy-sarif | tee $out | sarif-fmt
            '';

            inherit (self.packages.${system}.nrr) cargoDeps;

            nativeBuildInputs = with pkgs; [
              rustPlatform.cargoSetupHook
              cargo
              rustc
              clippy
              clippy-sarif
              sarif-fmt
            ];
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
            ];

            inputsFrom = [ self.packages.${system}.nrr ];

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
          packages = self.overlays.default null pkgs;
        in
        {
          inherit (packages) nrr;
          default = packages.nrr;
        }
      );

      legacyPackages = forAllSystems (
        system: nixpkgsFor.${system}.callPackage ./nix/static.nix { inherit self; }
      );

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);

      overlays.default = _: prev: {
        nrr = prev.callPackage ./nix/package.nix { inherit self; };
      };
    };
}
