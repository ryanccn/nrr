# nrr

[![Crates.io Version](https://img.shields.io/crates/v/nrr?style=flat-square&logo=rust)](https://crates.io/crates/nrr) [![Crates.io Total Downloads](https://img.shields.io/crates/d/nrr?style=flat-square&logo=rust)](https://crates.io/crates/nrr) [![GitHub Actions Build Workflow Status](https://img.shields.io/github/actions/workflow/status/ryanccn/nrr/build.yml?branch=main&event=push&style=flat-square&logo=github)](https://github.com/ryanccn/nrr/actions/workflows/build.yml) [![GitHub Actions Build Workflow Status](https://img.shields.io/github/actions/workflow/status/ryanccn/nrr/test.yml?branch=main&event=push&style=flat-square&logo=github&label=test)](https://github.com/ryanccn/nrr/actions/workflows/test.yml) [![GitHub License](https://img.shields.io/github/license/ryanccn/nrr?style=flat-square&color=blue)](https://github.com/ryanccn/nrr/blob/main/LICENSE)

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

Minimal, blazing fast npm scripts runner.

[**Features**](#features) • [**Installation**](#installation) • [**Usage**](#usage)

## Features

### Performance

nrr initializes and resolves scripts way faster than other package managers and script runners. It achieves this by providing the largest feature coverage possible while keeping it simple and performant.

<details>

<summary>Benchmark of <code>nrr</code>, <code>npm</code>, <code>yarn</code>, <code>pnpm</code>, <code>node --run</code>, <code>bun</code>, and <code>dum</code> running a simple <code>true</code> script</summary>

| Command      |    Mean [ms] | Min [ms] | Max [ms] |       Relative |
| :----------- | -----------: | -------: | -------: | -------------: |
| **`nrr`**    |    2.2 ± 0.3 |      1.9 |      7.7 |           1.00 |
| `dum`        |    2.7 ± 0.2 |      2.4 |      5.2 |    1.22 ± 0.17 |
| `bun`        |    7.1 ± 0.3 |      6.5 |     10.3 |    3.21 ± 0.40 |
| `node --run` |   41.6 ± 1.8 |     39.3 |     49.1 |   18.80 ± 2.35 |
| `npm`        | 261.7 ± 17.1 |    245.5 |    292.2 | 118.36 ± 15.91 |
| `yarn`       | 305.7 ± 14.6 |    295.2 |    343.9 | 138.28 ± 17.52 |
| `pnpm`       | 502.7 ± 10.7 |    489.8 |    522.9 | 227.40 ± 27.12 |

<small>Benchmarks run on an AWS EC2 `t4g.micro` instance with the command <code>hyperfine --shell=none --warmup=5 --output=pipe --export-markdown=benchmark.md 'npm run dev' -n 'npm' 'yarn run dev' -n 'yarn' 'pnpm run dev' -n 'pnpm' 'node --run dev' -n 'node --run' 'bun run dev' -n 'bun' 'dum run dev' -n 'dum' 'nrr dev' -n 'nrr'</code></small>

</details>

### Package and script metadata display

nrr provides a better-looking display of package details and the command being run than most, and also prints this information to `stderr` instead of `stdout` like some of the package managers do (erroneously).

### Command execution

On top of the standard script runner functionality that runs your scripts in `package.json`, nrr can also execute arbitrary commands in your npm package environments! You can use the `nrr exec` and `nrr x` commands to execute commands, similar to how `npx` or `pnpm exec` works (but faster, of course).

Do note, however, that nrr cannot run commands from remote packages! That feature falls within the purview of package managers, which nrr is not.

> [!TIP]
>
> If you create a symlink that has a name of `nrx` (or, on Windows, a hard link that has a name of `nrx.exe`) in your `PATH`, you can execute commands through the `nrx` binary without using a subcommand!

### Script listing

Running nrr without any arguments or running the `nrr list` subcommand will try to find any packages in the current working directory and its ancestors, and list the scripts available from them, both name and command.

### Tooling compatibility

nrr has compatibility functionality that patches `npm_execpath` so that tools like [`npm-run-all2`](https://github.com/bcomnes/npm-run-all2) use it instead of package managers for running sub-scripts.

> [!WARNING]
>
> This may cause unexpected behavior when `npm_execpath` is used for non-script running purposes, so open an issue if you encounter any bugs.

When running nested scripts with nrr, nrr has specialized behavior that prints extra information while staying minimal and performant:

```
sveltekit-project@0.0.1
$ run-s lint format:check
sveltekit-project@0.0.1 lint
$$ eslint .
```

### Spelling suggestions

If you mistype a script name (e.g. `buils` instead of `build`), nrr will intelligently suggest the right script to run in the error message using the Jaro similarity algorithm from the [`strsim`](https://docs.rs/strsim/latest/strsim/fn.jaro.html) crate.

## Installation

### Nix

Add the overlay or package from the `github:ryanccn/nrr` flake to your own system flake. Alternatively, install the package declaratively:

```console
$ nix profile install 'github:ryanccn/nrr#nrr'
```

nrr is also available in [Nixpkgs](https://github.com/NixOS/nixpkgs) as `nixpkgs#nrr`.

### Cargo

nrr supports [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), which downloads a binary if possible from GitHub Releases instead of compiling the crate from source.

```console
$ cargo binstall nrr
```

If you do want to compile from source, install with Cargo directly:

```console
$ cargo install nrr
```

Or if you want to stay on the bleeding edge, install with Cargo from the Git repository:

```console
$ cargo install --git https://github.com/ryanccn/nrr.git
```

### GitHub Releases

You can download binaries pre-compiled for Linux, macOS, and Windows from the [latest GitHub Release](https://github.com/ryanccn/nrr/releases/latest). Linux binaries are statically linked to musl; Windows binaries require MSVC.

## Usage

```console
$ nrr dev
$ nrr run dev
```

```console
$ nrx eslint --help
$ nrr x eslint --help
$ nrr exec eslint --help
```

```console
$ nrr
$ nrr list
```

This section provides an overview of nrr's command-line functionality. For more options and information, run `nrr --help`!

## License

GPLv3
