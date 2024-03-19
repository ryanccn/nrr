{
  lib,
  pkgsStatic,
  crane,
  fenix,
  nrr,
}: let
  target = pkgsStatic.stdenv.hostPlatform.config;
  targetUpper = lib.toUpper (builtins.replaceStrings ["-"] ["_"] target);
  inherit (pkgsStatic.stdenv) cc;

  toolchain = with fenix;
    combine [
      minimal.cargo
      minimal.rustc
      targets.${target}.latest.rust-std
    ];

  crane' = crane.overrideToolchain toolchain;
in
  (nrr.override {
    crane = crane';
    lto = true;
  })
  .overrideAttrs (old: {
    nativeBuildInputs = old.nativeBuildInputs ++ [pkgsStatic.stdenv.cc];

    # we may be cross compiling here, so there's
    # no guarntee we can actually run the built binary
    doCheck = false;

    env = {
      CARGO_BUILD_TARGET = target;
      CARGO_BUILD_RUSTFLAGS = old.env.CARGO_BUILD_RUSTFLAGS + " -C target-feature=+crt-static";
      "CARGO_TARGET_${targetUpper}_LINKER" = "${cc}/bin/${cc.targetPrefix}cc";
    };
  })
