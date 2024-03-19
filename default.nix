{
  lib,
  stdenv,
  pkg-config,
  rustPlatform,
  CoreFoundation,
  Security,
  IOKit,
  libiconv,
  version,
  self,
  lto ? true,
  optimizeSize ? true,
  nrxAlias ? true,
}: let
  filter = path: type: let
    path' = toString path;
    base = baseNameOf path';

    matches = lib.any (suffix: lib.hasSuffix suffix base) [".rs" ".toml"];
    isLock = base == "Cargo.lock";
  in
    type == "directory" || matches || isLock;

  filterSource = src:
    lib.cleanSourceWith {
      src = lib.cleanSource src;
      inherit filter;
    };
in
  rustPlatform.buildRustPackage {
    pname = "nrr";
    inherit version;

    src = filterSource self;
    cargoLock.lockFile = ./Cargo.lock;

    RUSTFLAGS =
      lib.optionalString lto " -C lto=fat -C embed-bitcode=yes"
      + lib.optionalString optimizeSize " -C codegen-units=1 -C strip=symbols -C opt-level=z";

    buildInputs = lib.optionals stdenv.isDarwin [
      CoreFoundation
      Security
      IOKit
      libiconv
    ];

    nativeBuildInputs = [
      pkg-config
    ];

    postInstall = ''
      ${lib.optionalString nrxAlias "ln -s $out/bin/nrr $out/bin/nrx"}
    '';

    meta = with lib; {
      description = "Minimal, blazing fast npm scripts runner";
      maintainers = with maintainers; [ryanccn];
      license = licenses.gpl3Only;
      mainProgram = "nrr";
    };
  }
