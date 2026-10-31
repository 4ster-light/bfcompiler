# [BFCompiler](https://bfcompiler.pages.dev)

[![Version](https://img.shields.io/badge/version-1.0.0-2f6f4e?style=flat-square)](Cargo.toml)
[![Rust](https://img.shields.io/badge/rust-1.99.1-dea584?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org)
[![Nix flake](https://img.shields.io/badge/Nix-flake-5277C3?logo=nixos&logoColor=white&style=flat-square)](flake.nix)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-Wasm-654FF0?logo=webassembly&logoColor=white&style=flat-square)](crates/web)
[![Website](https://img.shields.io/badge/website-bfcompiler.pages.dev-1f7a5a?logo=cloudflare&logoColor=white&style=flat-square)](https://bfcompiler.pages.dev)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Sponsor](https://img.shields.io/badge/Sponsor-GitHub%20Sponsors-EA4AAA?logo=githubsponsors&logoColor=white&style=flat-square)](https://github.com/sponsors/4ster-light)
[![Web tests](https://github.com/4ster-light/bfcompiler/actions/workflows/test.yml/badge.svg)](https://github.com/4ster-light/bfcompiler/actions/workflows/test.yml)
[![Deploy](https://github.com/4ster-light/bfcompiler/actions/workflows/pages.yml/badge.svg)](https://github.com/4ster-light/bfcompiler/actions/workflows/pages.yml)

A small Brainfuck toolchain written in Rust: a command-line interpreter, a compiler, a terminal REPL, and a browser playground built with WebAssembly.

> [Brainfuck](https://esolangs.org/wiki/Brainfuck) was created by Urban Müller in 1993.

## Table of Contents

- [Workspace](#workspace)
- [Quick start with Nix](#quick-start-with-nix)
  - [How the web dependencies are handled](#how-the-web-dependencies-are-handled)
- [CLI usage](#cli-usage)
- [Web playground](#web-playground)
- [Development checks](#development-checks)
- [License](#license)

## Workspace

| Path                 | Purpose                                                           |
|----------------------|-------------------------------------------------------------------|
| `.` / `bf`           | The `bf` command-line application                                 |
| `crates/compiler`    | Brainfuck → Rust source → native executable                       |
| `crates/interpreter` | Portable, resumable interpreter with no platform I/O dependencies |
| `crates/repl`        | Interactive terminal REPL                                         |
| `crates/web`         | `wasm-bindgen` bindings used by the playground                    |
| `web`                | TypeScript/Vite static playground, managed with pnpm              |

The interpreter validates brackets before execution, uses 30,000 wrapping 8-bit cells, reads `0` at end of input, and supports instruction budgets so non-terminating programs can be paused safely.

## Quick start with Nix

The flake provides the CLI and one complete web output. Run these from the repository root:

```bash
nix develop
```

Inside the development shell:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings

cd web
pnpm install
pnpm dev          # http://localhost:5173
```

Outside the shell:

```bash
nix build .#bf        # CLI; also the default package
nix build .#web       # Wasm bindings + static site in ./result
nix run .#web         # serve the built site on http://localhost:4173
nix flake check       # packages, tests, Clippy, formatting
nix fmt               # format Nix files with nixfmt
```

`nix flake check` runs these checks: `bf`, `web`, `rustfmt`, `clippy`, `tests`, `nixfmt`, `web-check`, and `smoke`.

`nix run .#web` serves the bundled files with nixpkgs' `static-server`; it does not start a Vite dev server. To use another port:

```bash
PORT=8080 nix run .#web
```

The dev shell uses the stable Rust toolchain from rust-overlay with Clippy, rustfmt, and the `wasm32-unknown-unknown` target. It also provides Node.js 26, pnpm 12, `wasm-pack`, `wasm-bindgen-cli`, `lld`, nixfmt, and Biome.

> **Nix and untracked files:** a flake only sees files tracked by Git. Run `git add` for new crates or web files before `nix build` / `nix flake check`. While iterating on untracked files you can use `nix build path:.#web` instead.

### How the web dependencies are handled

The flake uses the standard Nixpkgs pnpm path:

- `fetchPnpmDeps` fetches exactly `web/package.json`, `web/pnpm-lock.yaml`, and `web/pnpm-workspace.yaml` into a fixed-output store.
- `pnpmConfigHook` installs those dependencies offline during the build.
- Crane builds the Wasm bindings and the Vite bundle as one `web` derivation. There is no separate `wasm` flake output.
- Only a lockfile or manifest change requires updating the `pnpmDeps` hash in `flake.nix`.

Other reasonable workflows, depending on what you need:

| Workflow              | Command                                                     | Trade-off                                                       |
|-----------------------|-------------------------------------------------------------|-----------------------------------------------------------------|
| Hermetic Nix build    | `nix build .#web`                                           | Reproducible, but needs a hash update when pnpm deps change     |
| Fast UI iteration     | `nix develop`, then `pnpm dev`                              | No Nix rebuild; uses the normal pnpm store                      |
| CI / Cloudflare Pages | `pnpm install --frozen-lockfile && pnpm wasm && pnpm build` | No Nix required; depends on the CI-provided Rust/Wasm toolchain |
| Plain static output   | `pnpm wasm && pnpm build`                                   | Produces `web/dist`; serve it with any static file server       |

The flake deliberately does not vendor `node_modules`, run an impure network install during a Nix build, or use `buildNpmPackage` (which expects an npm lockfile).

## CLI usage

```bash
bf run hello.bf
bf build hello.bf
bf build hello.bf --save  # Keep the generated Rust source
bf repl                   # Type exit to leave the REPL
```

The example in [`hello.bf`](hello.bf) prints `Hello World!`:

```brainfuck
++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.
```

Install the CLI with Nix or Cargo:

```bash
nix profile add github:4ster-light/bfcompiler
# or
cargo install --git https://github.com/4ster-light/bfcompiler.git
```

## Web playground

See [`web/README.md`](web/README.md) for browser tests and Cloudflare Pages details. Pushes to `main` that touch the playground deploy to <https://bfcompiler.pages.dev> through `.github/workflows/pages.yml`.

## Development checks

```bash
nix develop -c cargo fmt --all --check
nix develop -c cargo clippy --workspace --all-targets -- -D warnings
nix develop -c cargo test --workspace

cd web
nix develop -c pnpm install --frozen-lockfile
nix develop -c pnpm test
nix develop -c pnpm wasm && pnpm build
```

For a Nix-only verification, run `nix flake check`. Format the flake with `nix fmt`.

## License

MIT
