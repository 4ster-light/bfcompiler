# BFCompiler

A small Brainfuck toolchain written in Rust: a command-line interpreter, a compiler, a terminal REPL, and a browser playground built with WebAssembly.

[Brainfuck](https://esolangs.org/wiki/Brainfuck) was created by Urban Müller in 1993.

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

`nix flake check` runs these checks: `bf`, `web`, `rustfmt`, `clippy`, `tests`, `nixfmt`, `web-format`, and `smoke`.

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

| Workflow | Command | Trade-off |
| --- | --- | --- |
| Hermetic Nix build | `nix build .#web` | Reproducible, but needs a hash update when pnpm deps change |
| Fast UI iteration | `nix develop`, then `pnpm dev` | No Nix rebuild; uses the normal pnpm store |
| CI / Cloudflare Pages | `pnpm install --frozen-lockfile && pnpm build` | No Nix required; depends on the CI-provided Rust/Wasm toolchain |
| Plain static output | `pnpm wasm && pnpm build` | Produces `web/dist`; serve it with any static file server |

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

The playground has example programs, program input, run/pause/step controls, a live memory tape, and output download/copy actions. It runs entirely in the browser and defaults to a dark theme; the header switch changes to light and remembers the choice.

To develop without the Nix shell:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

cd web
pnpm install
pnpm dev
```

`pnpm dev` rebuilds the WebAssembly bindings through its `predev` hook. `pnpm build` only runs Vite, so use `pnpm wasm && pnpm build` for a complete production bundle. Other useful commands:

```bash
pnpm test             # Playwright UI tests
pnpm fmt              # Biome formatting
pnpm lint             # Biome checks with safe fixes
pnpm wasm && pnpm build
```

See [`web/README.md`](web/README.md) for browser tests and future Cloudflare Pages settings. No deployment happens automatically.

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

For a Nix-only verification, run `nix flake check`. Format the flake with `nix fmt` or `nixfmt flake.nix`.

## License

MIT
