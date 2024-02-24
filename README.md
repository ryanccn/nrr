# `nrr`

Minimal, blazing fast npm scripts runner.

## Why?

`nrr` initializes and resolves scripts way faster than package managers. It achieves this by providing the largest feature coverage possible while keeping it simple and performant.

<details>

<summary>Benchmark of <code>nrr</code>, <code>npm</code>, <code>yarn</code>, <code>pnpm</code>, <code>bun</code>, and <code>dum</code> running a simple <code>echo</code> script</summary>

| Command   |   Mean [ms] | Min [ms] | Max [ms] |     Relative |
| :-------- | ----------: | -------: | -------: | -----------: |
| **`nrr`** |   6.1 ± 0.3 |      5.6 |      8.2 |  1.17 ± 0.09 |
| `dum`     |   5.2 ± 0.3 |      4.9 |      6.1 |         1.00 |
| `bun`     |   7.8 ± 0.3 |      7.3 |      9.0 |  1.50 ± 0.10 |
| `yarn`    | 152.7 ± 0.9 |    151.0 |    154.8 | 29.39 ± 1.51 |
| `npm`     | 162.2 ± 1.3 |    159.9 |    164.8 | 31.21 ± 1.62 |
| `pnpm`    | 223.8 ± 2.7 |    220.4 |    231.3 | 43.07 ± 2.27 |

<small><code>hyperfine --shell=none --warmup=5 --output=pipe --export-markdown=benchmark.md 'npm run dev' -n 'npm' 'yarn run dev' -n 'yarn' 'pnpm run dev' -n 'pnpm' 'bun run dev' -n 'bun' 'dum run dev' -n 'dum' 'nrr dev' -n 'nrr'</code></small>

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
