{
  lib,
  stdenv,
  rustPlatform,
  darwin,
  pkg-config,
  # this doesn't work with a default toolchain
  lto ? false,
  optimizeSize ? false,
  nrxAlias ? true,
}:
rustPlatform.buildRustPackage rec {
  pname = passthru.cargoToml.package.name;
  inherit (passthru.cargoToml.package) version;

  __structuredAttrs = true;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.lock
      ../Cargo.toml
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
    CoreFoundation
    Security
    IOKit
    darwin.libiconv
  ]);

  nativeBuildInputs = lib.optionals stdenv.isDarwin [
    pkg-config
  ];

  env =
    lib.optionalAttrs lto {
      CARGO_PROFILE_RELEASE_LTO = "fat";
    }
    // lib.optionalAttrs optimizeSize {
      CARGO_PROFILE_RELEASE_OPT_LEVEL = "z";
      CARGO_PROFILE_RELEASE_PANIC = "abort";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
      CARGO_PROFILE_RELEASE_STRIP = "symbols";
    };

  postInstall = lib.optionalString nrxAlias "ln -s $out/bin/nr{r,x}";

  passthru = {
    cargoToml = lib.importTOML ../Cargo.toml;
  };

  meta = with lib; {
    description = "Minimal, blazing fast npm scripts runner";
    maintainers = with maintainers; [ryanccn];
    license = licenses.gpl3Only;
    mainProgram = "nrr";
  };
}
