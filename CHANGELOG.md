# Changelog

All notable, and not so notable, changes to this project will be documented in this file.

## [0.9.1] - 2024-05-11

### Features

- [`c360fbe`](https://github.com/ryanccn/nrr/commit/c360fbe71fa39632c830c6d9d8e6ff38e1c1e3dd) Add installer script - Ryan Cao

### Bug Fixes

- [`7d81dad`](https://github.com/ryanccn/nrr/commit/7d81dada2a2b3ba4c529aa24834bf25ebb354b31) (_list_) Remove extra newline at end of output - Ryan Cao

### Refactor

- [`9235f68`](https://github.com/ryanccn/nrr/commit/9235f68db24aceb9e76ccfeea7e247344348f544) Make shared util modules - Ryan Cao

### Testing

- [`7fe789c`](https://github.com/ryanccn/nrr/commit/7fe789cefdad8917d792fd0fae241e7bdf356340) (_run_) Add test for multiple arguments - Ryan Cao

### Documentation

- [`7a46bb6`](https://github.com/ryanccn/nrr/commit/7a46bb632f3dbdc89fddd9f20ff07ab3ffbf2e64) Complete GitHub Releases section - Ryan Cao
- [`16ab9b9`](https://github.com/ryanccn/nrr/commit/16ab9b995629cb654dc246d98305793129be8e7e) Add `cargo-binstall` support - Ryan Cao
- [`e8d5c80`](https://github.com/ryanccn/nrr/commit/e8d5c8075d755e0e65457608e6f57de04bc5aaad) Update benchmark to include `node --run` - Ryan Cao

### Build

- [`122bda0`](https://github.com/ryanccn/nrr/commit/122bda048a3c4811fa966fd0ee220fbecb5ec113) Add `aarch64-pc-windows-msvc` target - Ryan Cao

## [0.9.0] - 2024-04-19

### Features

- [`7c1576e`](https://github.com/ryanccn/nrr/commit/7c1576e939bb07704d4e654f424a8b498ec4b418) [**breaking**] Arbitrary command execution (#27) - Ryan Cao
- [`b6d0d14`](https://github.com/ryanccn/nrr/commit/b6d0d14fc49de19db6a109a6e651e6e93bdd18f2) Add `silent` flag - Ryan Cao
- [`e3ebf16`](https://github.com/ryanccn/nrr/commit/e3ebf165a08571d341ce3b717a0cea49050046bd) Adopt `simd_json`, drop zero-copy - Ryan Cao
- [`0adae88`](https://github.com/ryanccn/nrr/commit/0adae88811096ded12a6e028c5c8ee829579d21e) Add binary artifacts and static builds (#31) - seth
- [`fd8e1de`](https://github.com/ryanccn/nrr/commit/fd8e1dee6658473de4d1c4ce1a25355945e122ea) Load environment files with `-e/--env-file` - Ryan Cao
- [`6aa164f`](https://github.com/ryanccn/nrr/commit/6aa164fc4f9bd1c3fedfcbf3b270af4e1496a0e9) (_exec_) Allow non-npm installed commands, adopt `execvp` on unix - Ryan Cao

### Bug Fixes

- [`ff7320e`](https://github.com/ryanccn/nrr/commit/ff7320ec7b7876514891f193cc95857efee37133) (_exec_) Never print information - Ryan Cao

### Refactor

- [`ed04295`](https://github.com/ryanccn/nrr/commit/ed04295ff662f1e2fc497e6e73c5d6650b1d2a32) Reduce duplication and improve organization - Ryan Cao

### Performance

- [`6d187e6`](https://github.com/ryanccn/nrr/commit/6d187e6c6bfeecea6239b07c0ba23004d87dd9e9) Use `itoa` for integer formatting - Ryan Cao

### Testing

- [`a8a4859`](https://github.com/ryanccn/nrr/commit/a8a485922abd742bd6f4acd9c50e11d1a72ca7ab) Add integration tests - Ryan Cao

### Documentation

- [`7eba97b`](https://github.com/ryanccn/nrr/commit/7eba97bc83cfaca9e2ecaaa1e874b4d7d4e897f4) Fix admonition syntax - Ryan Cao

### Miscellaneous

- [`06ec194`](https://github.com/ryanccn/nrr/commit/06ec194a0447f449c3457234d0cd7c0423c7f9d2) Adopt `cargo-auditable` - Ryan Cao
- [`d622834`](https://github.com/ryanccn/nrr/commit/d622834916e5a9e7973bfcd577697675879f929e) Remove unused fns - Ryan Cao

### Revert

- [`10aaa49`](https://github.com/ryanccn/nrr/commit/10aaa49a33f4daa223a46da1c03df1396e18019c) Don't use `exec` on unix - Ryan Cao

## [0.8.1] - 2024-02-24

### Bug Fixes

- [`f0ed577`](https://github.com/ryanccn/nrr/commit/f0ed577b5df48b81eb3331ca8389fc2339c36442) Inverted suggestions sorting - Ryan Cao

### Documentation

- [`5c09207`](https://github.com/ryanccn/nrr/commit/5c09207426ab432cb48703a6d63d97ac128cfb4b) Explain features - Ryan Cao

## [0.8.0] - 2024-02-24

### Features

- [`79aefab`](https://github.com/ryanccn/nrr/commit/79aefabb74e6f5cd459fdab853fb73c785e046b2) Add spelling suggestions - Ryan Cao

### Bug Fixes

- [`a38d15d`](https://github.com/ryanccn/nrr/commit/a38d15da479c63910235e611de5ce2866b4de128) Remove extra newline when listing - Ryan Cao
- [`e52fded`](https://github.com/ryanccn/nrr/commit/e52fded8d970f344311f9c66fb0d7277fba7185a) Adopt `serde_with` - Ryan Cao

### Performance

- [`2876e81`](https://github.com/ryanccn/nrr/commit/2876e81ef4903bb49e0de24e96fb928f25624ea4) Remove async runtime (Tokio) - Ryan Cao

### Miscellaneous

- [`123e864`](https://github.com/ryanccn/nrr/commit/123e864ca95b876ed8195d1c38c3d50b21efe3f8) Update benchmarks - Ryan Cao

## [0.7.0] - 2024-02-21

### Features

- [`922dbb1`](https://github.com/ryanccn/nrr/commit/922dbb10a1f21c405901138d5eb9cdf9980b2ed3) [**breaking**] Allow hyphen values for extra arguments - Ryan Cao

### Performance

- [`2cea6d5`](https://github.com/ryanccn/nrr/commit/2cea6d5206f24ac931270c102bece1e79ad6d8a7) Use AES-backed hashing - Ryan Cao
- [`d6ba0c1`](https://github.com/ryanccn/nrr/commit/d6ba0c11b8b1f6914e45a8866cdbbdfc26dae473) Reduce unnecessary allocs when constructing arguments - Ryan Cao
- [`5322336`](https://github.com/ryanccn/nrr/commit/5322336981e803b9805d776f6509faaa522ca7cf) Adopt `smartstring` to automatically inline small strings - Ryan Cao

## [0.6.0] - 2024-02-20

### Performance

- [`abad8b7`](https://github.com/ryanccn/nrr/commit/abad8b7b6a6e031f9d69b5d278e13772d14a8c47) Use `OnceLock` to cache script level - Ryan Cao
- [`f5b9727`](https://github.com/ryanccn/nrr/commit/f5b97273ed931e9a687ccfac11c776fbcdbdfc25) Implement zero-copy correctly - Ryan Cao

## [0.5.2] - 2024-02-04

### Bug Fixes

- [`065f285`](https://github.com/ryanccn/nrr/commit/065f2854b76ef3799d29cec83f5e2c042243d681) Use `Cow` in `PackageJson` struct - Ryan Cao

## [0.5.1] - 2024-01-28

### Refactor

- [`e9293dd`](https://github.com/ryanccn/nrr/commit/e9293dd78a69e02821aea9ae388a3a3d571f835a) Improve logic, cleanup - Ryan Cao

### Documentation

- [`db71ba5`](https://github.com/ryanccn/nrr/commit/db71ba5c58a81bf6211c4e97c1a9b1f2eb8556f9) Remove code block languages in usage [skip ci] - Ryan Cao

## [0.5.0] - 2024-01-26

### Removed

- [`2a64c47`](https://github.com/ryanccn/nrr/commit/2a64c472602ef0634000a361ebe4f0cf7595d752) [**breaking**] Compatibility modes - Ryan Cao

### Documentation

- [`79133da`](https://github.com/ryanccn/nrr/commit/79133da713313d98bcdcc061ff49619350dc66a5) Update README - Ryan Cao

### Miscellaneous

- [`be8c177`](https://github.com/ryanccn/nrr/commit/be8c177639b0ca15ac4e4fd91554e392651eb441) Update flake inputs - Ryan Cao

## [0.4.1] - 2024-01-24

### Features

- [`0713169`](https://github.com/ryanccn/nrr/commit/07131690cf3ee1e780077392e422aaaea5c29a5e) Add exit status logging - Ryan Cao

### Refactor

- [`97b61c3`](https://github.com/ryanccn/nrr/commit/97b61c3726dbaed7c241b85a4a90f86c268f2e31) Don't read package again when serializing to environment - Ryan Cao

### Miscellaneous

- [`74f1061`](https://github.com/ryanccn/nrr/commit/74f1061478941c604a385fc3e3032042826e4704) Add garnix config - Ryan Cao
- [`59fac25`](https://github.com/ryanccn/nrr/commit/59fac25986cc9bc7108954a81a84a42ed2cc7378) Remove Nix workflow in favor of garnix - Ryan Cao

## [0.4.0] - 2024-01-23

### Features

- [`b8d8c4a`](https://github.com/ryanccn/nrr/commit/b8d8c4a8837a7ad43200b28b56525c9c0c6667f2) Track script level and log script name in nested scripts - Ryan Cao

## [0.3.1] - 2023-12-25

### Features

- [`73924d9`](https://github.com/ryanccn/nrr/commit/73924d90dff5373c0f47904c1568492d6b7a78a3) Print help if no scripts are found - Ryan Cao

### Refactor

- [`318d5b1`](https://github.com/ryanccn/nrr/commit/318d5b1ca674f2fa3f2df9a8299fbc55ea559ecc) Improve iterators - Ryan Cao

## [0.3.0] - 2023-12-18

### Bug Fixes

- [`7a166f0`](https://github.com/ryanccn/nrr/commit/7a166f0c8dca85eaabe506fd93111666d809ee34) Handle pre/post script prefixes properly - Ryan Cao

### Refactor

- [`c4a32f5`](https://github.com/ryanccn/nrr/commit/c4a32f589b656f6cbf56bd9c4b3ee44cad33c5ed) Use `color_eyre` - Ryan Cao

## [0.2.0] - 2023-11-25

### Features

- [`cd0aea7`](https://github.com/ryanccn/nrr/commit/cd0aea7f1577c05161295f90c202f6b65e5d7a49) Initial commit - Ryan Cao
- [`819be15`](https://github.com/ryanccn/nrr/commit/819be15cebcc4f0caf39c7a7fc48d5292134e1dd) Add script listing - Ryan Cao

### Bug Fixes

- [`7483b4d`](https://github.com/ryanccn/nrr/commit/7483b4dacd537bd484377e9023179bc24179fc3b) Ensure that `NRR_COMPAT_MODE` is not empty - Ryan Cao
- [`b70afec`](https://github.com/ryanccn/nrr/commit/b70afec1e5adb55454c42ae4993476ded0221850) Handle errors on `Child::wait` - Ryan Cao
- [`ce8e4fb`](https://github.com/ryanccn/nrr/commit/ce8e4fb921b33283dcedb44e78a6aa312a3a0a49) Allow run-script in nested compat mode - Ryan Cao
- [`efa2e97`](https://github.com/ryanccn/nrr/commit/efa2e97a9c7e4da3114c8f33cd0e5508fe4aac56) Only handle unix signals on unix - Ryan Cao
- [`733288e`](https://github.com/ryanccn/nrr/commit/733288e58416a7b9337952bf41e0d4a0f8888877) Sort scripts and display in normal text - Ryan Cao

### Refactor

- [`d2d6afe`](https://github.com/ryanccn/nrr/commit/d2d6afec34a5c93b2f90d5c3e86ddec4ff3a4568) Search packages and resolve node modules incrementally - Ryan Cao
- [`5f6ede8`](https://github.com/ryanccn/nrr/commit/5f6ede88e911868146acb35fb8af39543354ddf1) Optionally run pre and post scripts - Ryan Cao

### Performance

- [`58d45d0`](https://github.com/ryanccn/nrr/commit/58d45d08ddeab4cf09b53c7240b2a14f618399aa) Only include `ctrl_c` on unix - Ryan Cao

### Documentation

- [`2d6ec80`](https://github.com/ryanccn/nrr/commit/2d6ec80884f74d33c1296d57aa826887cba1a9f2) Add cargo git install [skip ci] - Ryan Cao
- [`93bebed`](https://github.com/ryanccn/nrr/commit/93bebeda5422731dbda362a05e9fad25d1af1edf) Use published crate - Ryan Cao

### Styling

- [`d3db9c1`](https://github.com/ryanccn/nrr/commit/d3db9c1fc1d2e0ae5aaa9dca2dc780cadc476164) `searched_dirs` → `searched_pkgs` - Ryan Cao

### Miscellaneous

- [`1373b97`](https://github.com/ryanccn/nrr/commit/1373b972de7540e9496217b3121455b4eddb83dd) Add cargo release workflow - Ryan Cao

[0.9.1]: https://github.com/ryanccn/nrr/compare/v0.9.0..0.9.1
[0.9.0]: https://github.com/ryanccn/nrr/compare/v0.8.1..v0.9.0
[0.8.1]: https://github.com/ryanccn/nrr/compare/v0.8.0..v0.8.1
[0.8.0]: https://github.com/ryanccn/nrr/compare/v0.7.0..v0.8.0
[0.7.0]: https://github.com/ryanccn/nrr/compare/v0.6.0..v0.7.0
[0.6.0]: https://github.com/ryanccn/nrr/compare/v0.5.2..v0.6.0
[0.5.2]: https://github.com/ryanccn/nrr/compare/v0.5.1..v0.5.2
[0.5.1]: https://github.com/ryanccn/nrr/compare/v0.5.0..v0.5.1
[0.5.0]: https://github.com/ryanccn/nrr/compare/v0.4.1..v0.5.0
[0.4.1]: https://github.com/ryanccn/nrr/compare/v0.4.0..v0.4.1
[0.4.0]: https://github.com/ryanccn/nrr/compare/v0.3.1..v0.4.0
[0.3.1]: https://github.com/ryanccn/nrr/compare/v0.3.0..v0.3.1
[0.3.0]: https://github.com/ryanccn/nrr/compare/v0.2.0..v0.3.0
