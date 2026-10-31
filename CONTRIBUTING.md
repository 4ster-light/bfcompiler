# Contributing to BFCompiler

## Quick Start

```bash
nix develop
```

The shell includes the rust-overlay stable toolchain with Clippy, rustfmt, and the Wasm target, plus pnpm 12, Node.js 26, `wasm-pack`, `wasm-bindgen-cli`, `lld`, nixfmt, and Biome.

## Verify Changes

Rust:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Web playground:

```bash
cd web
pnpm install --frozen-lockfile
pnpm exec playwright install chromium  # first run only
pnpm test
pnpm wasm && pnpm build
pnpm check
pnpm fmt
pnpm lint
```

Nix:

```bash
nix flake check
nix build .#bf .#web
nix run .#web  # serves the built bundle on http://localhost:4173
nix fmt        # nixfmt
```

The checks are `bf`, `web`, `rustfmt`, `clippy`, `tests`, `nixfmt`, `web-check`, and `smoke`.

`nix build` only sees files tracked by Git, so run `git add` after adding new crates or web files. For a quick check before staging, `nix build path:.#web` includes untracked files.

## What To Keep In Mind

- Keep changes focused.
- Update docs when behavior, dependencies, or structure changes.
- Use pnpm for the web workspace; keep `pnpm-lock.yaml` up to date.
- Prefer the Nix flake for local development and validation.
- Mention any extra commands needed to verify your change.

## Need Help?

Open an issue before a larger refactor if you want to discuss the shape first.
