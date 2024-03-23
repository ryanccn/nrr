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

  env = {
    CARGO_BUILD_RUSTFLAGS = lib.concatStringsSep " " (
      lib.optionals lto [
        "-C lto=fat"
        "-C embed-bitcode=yes"
        "-Zdylib-lto"
      ]
      ++ lib.optionals optimizeSize [
        "-C codegen-units=1"
        "-C opt-level=z"
        "-C panic=abort"
        "-C strip=symbols"
      ]
    );
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
