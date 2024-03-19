{
  pkgsStatic,
  crane,
  fenix,
  nrr,
}: let
  target = pkgsStatic.stdenv.hostPlatform.config;

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
    env = {
      CARGO_BUILD_TARGET = target;
      CARGO_BUILD_RUSTFLAGS = old.env.CARGO_BUILD_RUSTFLAGS + " -C target-feature=+crt-static";
    };
  })
