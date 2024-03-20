{
  pkgsStatic,
  crane,
  rust-bin,
  nrr,
}: let
  target = pkgsStatic.stdenv.hostPlatform.config;
  inherit (pkgsStatic.stdenv) cc buildPlatform hostPlatform;

  staticToolchain = rust-bin.nightly.latest.minimal.override {
    extensions = ["rust-std"];
    targets = [target];
  };

  crane' = crane.overrideToolchain staticToolchain;
in
  (nrr.override {
    crane = crane';
    lto = true;
  })
  .overrideAttrs (old: {
    # don't run checks when cross compiling,
    # as we can't guarantee support for the target architecture
    doCheck = buildPlatform.system == hostPlatform.system;

    env = {
      CARGO_BUILD_TARGET = target;
      CARGO_BUILD_RUSTFLAGS = old.env.CARGO_BUILD_RUSTFLAGS + " -C target-feature=+crt-static -C linker=${cc}/bin/${cc.targetPrefix}cc";
    };
  })
