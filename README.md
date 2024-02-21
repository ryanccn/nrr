# `nrr`

Minimal, blazing fast npm scripts runner.

## Why?

`nrr` initializes and resolves scripts way faster than package managers. It achieves this by providing the largest feature coverage possible while keeping it simple and performant.

<details>

<summary>Benchmark of <code>nrr</code>, <code>npm</code>, <code>yarn</code>, <code>pnpm</code>, and <code>bun</code> running a simple <code>echo</code> script</summary>

| Command |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :------ | ----------: | -------: | -------: | -----------: |
| `nrr`   |   2.5 ± 0.2 |      2.3 |      3.3 |         1.00 |
| `dum`   |   5.3 ± 0.4 |      4.8 |      7.4 |  2.13 ± 0.21 |
| `bun`   |   7.8 ± 0.5 |      7.3 |     12.9 |  3.15 ± 0.30 |
| `yarn`  | 152.7 ± 0.8 |    151.0 |    154.3 | 61.41 ± 4.32 |
| `npm`   | 160.0 ± 1.2 |    157.9 |    162.4 | 64.35 ± 4.54 |
| `pnpm`  | 223.7 ± 0.9 |    222.2 |    225.6 | 89.94 ± 6.32 |

<small><code>hyperfine --shell=none --warmup=5 --export-markdown=benchmark.md 'dum run test' -n dum 'pnpm run test' -n pnpm 'yarn run test' -n yarn 'npm run test' -n npm 'bun run test' -n bun 'nrr run test' -n nrr</code></small>

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
$ cargo install nrr
```

```console
$ cargo install --git https://github.com/ryanccn/nrr.git
```

## Usage

```
$ nrr dev
```

```
sveltekit-project@0.0.1
$ vite dev

  VITE v5.0.12  ready in 10 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h + enter to show help
```

## Compatibility with other tools

`nrr` has compatibility functionality that patches `npm_execpath` so that tools like [`npm-run-all2`](https://github.com/bcomnes/npm-run-all2) use it instead of package managers for running sub-scripts. This may cause unexpected behavior when `npm_execpath` is used for non-script running purposes, so open an issue if you encounter any bugs.

In addition, when running nested scripts with `nrr` (i.e. running scripts with `nrr` through using tools like `npm-run-all2`), `nrr` has specialized behavior that prints extra information while staying minimal and performant:

```
sveltekit-project@0.0.1
$ run-s lint format:check
sveltekit-project@0.0.1 lint
$$ eslint .
```

## License

GPLv3
