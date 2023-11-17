# `nrr`

Minimal, blazing fast Node.js script runner.

## Why?

`nrr` initializes and resolves scripts way faster than package managers. It achieves this by providing the largest feature coverage possible while keeping it simple and performant.

<details>

<summary>Benchmark of <code>nrr</code>, <code>npm</code>, <code>yarn</code>, <code>pnpm</code>, and <code>bun</code> running a simple <code>echo</code> script</summary>

| Command |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :------ | ----------: | -------: | -------: | -----------: |
| `nrr`   |   5.7 ± 0.4 |      5.1 |      8.0 |         1.00 |
| `bun`   |   7.8 ± 0.5 |      7.2 |      9.4 |  1.37 ± 0.13 |
| `yarn`  | 146.3 ± 1.3 |    145.0 |    149.7 | 25.79 ± 1.93 |
| `npm`   | 159.9 ± 2.2 |    155.8 |    164.5 | 28.19 ± 2.13 |
| `pnpm`  | 223.1 ± 1.8 |    220.2 |    225.7 | 39.33 ± 2.94 |

</details>

In addition, `nrr` provides a better-looking display of project details and the command being run than most, and also prints this information to `stderr` instead of `stdout` like some of the package managers do (erroneously).

## Installation

### Nix

Add the overlay or package from the `github:ryanccn/nrr` flake to your own system flake. Alternatively, install the package declaratively:

```console
$ nix profile install 'github:ryanccn/nrr#nrr'
```

### Cargo

```console
$ cargo install --git https://github.com/ryanccn/nrr.git
```

### Build from source

```console
$ git clone https://github.com/ryanccn/nrr.git
$ cd nrr
$ cargo install --path .
```

## Compatibility modes

By providing a flag `-c/--compat {npm,yarn,pnpm,bun}` to `nrr` when running scripts, `nrr` can emulate the environments constructed by these package managers. Each package manager has a unique set of environment variables that they make available and `nrr` does its best to emulate them.

`npm_config_*` environment variables are not emulated. `nrr` is not a package manager.

## Compatibility with other tools

`nrr` has compatibility functionality that patches `npm_execpath` so that tools like [`npm-run-all2`](https://github.com/bcomnes/npm-run-all2) use it instead of package managers for running sub-scripts. This may cause unexpected behavior when `npm_execpath` is used for non-script running purposes, so open an issue if you encounter any bugs.

## License

GPLv3
