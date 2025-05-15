# nrr

[![Crates.io Version](https://img.shields.io/crates/v/nrr?style=flat-square&logo=rust)](https://crates.io/crates/nrr) [![Crates.io Total Downloads](https://img.shields.io/crates/d/nrr?style=flat-square&logo=rust)](https://crates.io/crates/nrr) [![GitHub Actions Build Workflow Status](https://img.shields.io/github/actions/workflow/status/ryanccn/nrr/build.yml?branch=main&event=push&style=flat-square&logo=github)](https://github.com/ryanccn/nrr/actions/workflows/build.yml) [![GitHub Actions Test Workflow Status](https://img.shields.io/github/actions/workflow/status/ryanccn/nrr/test.yml?branch=main&event=push&style=flat-square&logo=github&label=test)](https://github.com/ryanccn/nrr/actions/workflows/test.yml) [![GitHub License](https://img.shields.io/github/license/ryanccn/nrr?style=flat-square&color=blue)](https://github.com/ryanccn/nrr/blob/main/LICENSE)

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

Minimal, blazing fast npm scripts runner.

[**Features**](#features) • [**Installation**](#installation) • [**Usage**](#usage)

## Features

### Performance

nrr initializes and resolves scripts way faster than other package managers and script runners. It achieves this by providing the largest feature coverage possible while keeping it simple and performant.

<details>

<summary>Benchmark of <code>nrr</code>, <code>npm</code>, <code>yarn</code>, <code>pnpm</code>, <code>node --run</code>, <code>deno</code>, <code>bun</code>, and <code>dum</code> running a simple <code>true</code> script</summary>

| Command              |    Mean [ms] | Min [ms] | Max [ms] |       Relative |
| :------------------- | -----------: | -------: | -------: | -------------: |
| **`nrr` v0.10.2**    |    1.6 ± 0.1 |      1.4 |      2.5 |           1.00 |
| `dum` v0.1.20        |    2.0 ± 0.1 |      1.8 |      3.8 |    1.24 ± 0.13 |
| `bun` v1.2.12        |    4.2 ± 0.5 |      3.9 |     12.3 |    2.70 ± 0.38 |
| `node --run` v24.0.2 |    6.1 ± 0.4 |      5.4 |     11.2 |    3.86 ± 0.41 |
| `deno` v2.2.12       |    9.6 ± 0.4 |      8.9 |     12.4 |    6.11 ± 0.53 |
| `npm` v11.3.0        |  154.6 ± 6.9 |    147.5 |    169.5 |   98.13 ± 8.68 |
| `yarn` v4.9.1        |  306.1 ± 7.9 |    296.5 |    317.4 | 194.24 ± 15.64 |
| `pnpm` v10.11.0      | 463.2 ± 18.7 |    440.6 |    495.2 | 293.90 ± 25.38 |

<small>Benchmarks run on an AWS EC2 <code>t4g.micro</code> instance with the command <code>hyperfine --shell=none --warmup=5 --output=pipe --export-markdown=benchmark.md 'npm run dev' -n 'npm' 'yarn run dev' -n 'yarn' 'pnpm run dev' -n 'pnpm' 'node --run dev' -n 'node --run' 'bun run dev' -n 'bun' 'dum run dev' -n 'dum' 'nrr dev' -n 'nrr'</code></small>

</details>

### Package and script metadata display

nrr provides a better-looking display of package details and the command being run than most, and also prints this information to `stderr` instead of `stdout` like some of the package managers do (erroneously).

### Command execution

On top of the standard script runner functionality that runs your scripts in `package.json`, nrr can also execute arbitrary commands in your npm package environments! You can use the `nrr exec` and `nrr x` commands to execute commands, similar to how `npx` or `pnpm exec` works (but faster, of course).

Do note, however, that nrr cannot run commands from remote packages. That feature falls within the purview of package managers, which nrr is not.

> [!TIP]
>
> If you create a symlink that has a name of `nrx` (or, on Windows, a hard link that has a name of `nrx.exe`) in your `PATH`, you can execute commands through the `nrx` binary without using a subcommand!

### Script listing

Running nrr without any arguments or running the `nrr list` subcommand will try to find any packages in the current working directory and its ancestors, and list the scripts available from them, both name and command.

### Tooling compatibility

nrr has compatibility functionality that patches `npm_execpath` so that tools like [`npm-run-all2`](https://github.com/bcomnes/npm-run-all2) use it instead of package managers for running sub-scripts.

> [!WARNING]
>
> This may cause unexpected behavior when `npm_execpath` is used for non-script running purposes; open an issue if you encounter any bugs.

When running nested scripts with nrr, nrr has specialized behavior that prints nesting depth information while staying minimal and performant:

```
sveltekit-project@0.0.1
$ run-s lint format:check
sveltekit-project@0.0.1 lint
$$ eslint .
```

### Completions

nrr provides smart completions for Bash, Elvish, Fish, PowerShell, and Zsh. In addition to basic completions for subcommands and options, nrr also dynamically completes script names when using `nrr` / `nrr run` and executables when using `nrx` / `nrr exec`.

### Spelling suggestions

If you mistype a script name (e.g. `buils` instead of `build`), nrr will intelligently suggest the right script to run in the error message using the Jaro similarity algorithm from the [`strsim`](https://docs.rs/strsim/latest/strsim/fn.jaro.html) crate.

## Installation

### Nix

Add the overlay or package from the `github:ryanccn/nrr` flake to your own system flake. Alternatively, install the package declaratively:

```sh
nix profile install github:ryanccn/nrr
```

nrr is also available in [Nixpkgs](https://github.com/NixOS/nixpkgs) as `nixpkgs#nrr`.

### Installer

This installer script works on Linux and macOS and downloads binaries from GitHub Releases, falling back to `cargo install` if a prebuilt binary cannot be found.

```sh
curl --proto '=https' --tlsv1.2 https://nrr.ryanccn.dev | sh
```

### Cargo

nrr supports [cargo-binstall](https://github.com/cargo-bins/cargo-binstall), which downloads a binary if possible from GitHub Releases instead of compiling the crate from source.

```sh
cargo binstall nrr
```

If you do want to compile from source, install the crate with Cargo directly:

```sh
cargo install nrr
```

### GitHub Releases

You can download pre-compiled binaries for Linux, macOS, and Windows from the [latest GitHub Release](https://github.com/ryanccn/nrr/releases/latest). Linux binaries are statically linked to musl; Windows binaries require MSVC.

## Usage

```console
$ nrr dev
$ nrr run dev
```

```console
$ nrx eslint --version
$ nrr x eslint --version
$ nrr exec eslint --version
```

```console
$ nrr
$ nrr list
```

```bash
# Bash
echo "source <(COMPLETE=bash nrr)" >> ~/.bashrc

# Elvish
echo "eval (E:COMPLETE=elvish nrr | slurp)" >> ~/.elvish/rc.elv

# Fish
echo "source (COMPLETE=fish nrr | psub)" >> ~/.config/fish/config.fish

# PowerShell
$env:COMPLETE = "powershell"
echo "nrr | Out-String | Invoke-Expression" >> $PROFILE
Remove-Item Env:\COMPLETE

# Zsh
echo "source <(COMPLETE=zsh nrr)" >> ~/.zshrc
```

This section only provides an overview of nrr's command-line functionality. For more options and information, run `nrr --help`!

## License

GPLv3
