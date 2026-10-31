# Web playground

A static Brainfuck playground for the `bf-interpreter` crate. Rust is compiled to WebAssembly; the interface is TypeScript and Vite. Programs, input, and output stay in the browser. The page defaults to dark mode and remembers a light-theme choice in local storage.

[Brainfuck](https://esolangs.org/wiki/Brainfuck) was created by Urban Müller in 1993.

## Run with Nix

From the repository root:

```bash
nix develop
cd web
pnpm install
pnpm dev
```

Or build and serve the packaged static site:

```bash
nix run .#web               # http://localhost:4173
PORT=8080 nix run .#web     # another port
```

`nix build .#web` writes the complete Wasm + static site to `./result`. There is no separate Wasm flake output.

## Run without Nix

Requirements: Rust, the `wasm32-unknown-unknown` target, [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/), Node.js 22+, and pnpm 12+.

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

pnpm install
pnpm dev
```

Open the URL printed by Vite (normally `http://localhost:5173`). `pnpm dev` rebuilds the WebAssembly package first.

## Test and build

```sh
pnpm exec playwright install chromium  # one-time Chromium download
pnpm test
pnpm wasm && pnpm build
pnpm preview          # Vite preview
pnpm check            # Biome formatting + lint checks
pnpm fmt              # Biome formatting with fixes
pnpm lint             # Biome checks with safe fixes
```

The production static bundle is written to `dist/`. `pnpm build` only runs Vite; run `pnpm wasm` first when the Rust bindings changed. `nix run .#web` serves the Nix-built bundle with nixpkgs' `static-server`.

UI tests cover execution, UTF-8 input, stepping, errors, pausing, keyboard controls, the theme switch, desktop/mobile layouts, and WCAG audits in both themes.

## Nix dependency handling

The flake uses the hermetic Nixpkgs pnpm path: `fetchPnpmDeps` reads only `package.json`, `pnpm-lock.yaml`, and `pnpm-workspace.yaml`; `pnpmConfigHook` installs them offline; then Crane builds the Wasm bindings and the Vite bundle in one `web` output.

If you prefer a different workflow:

- **Local iteration:** use `nix develop`, then `pnpm install` / `pnpm dev`; no Nix rebuild or hash update.
- **CI or Cloudflare Pages:** run `pnpm install --frozen-lockfile && pnpm wasm && pnpm build` with Rust, the Wasm target, and `wasm-pack` available.
- **Static output check:** run `pnpm wasm && pnpm build` and serve `dist/` with any static server.

The flake does not vendor `node_modules`, perform an impure network install during a Nix build, or use `buildNpmPackage` with an npm lockfile.

## Cloudflare Pages

No deployment is performed by this repository. For a Pages project:

- **Root directory:** `web`
- **Build command:** `pnpm install --frozen-lockfile && pnpm wasm && pnpm build`
- **Build output directory:** `dist`
- **Node version:** 22 or newer
- **pnpm:** 12 or newer

The build environment must also provide Rust, `wasm32-unknown-unknown`, and `wasm-pack`. `pnpm wasm` regenerates the bindings, so the checked-in `src/wasm` output is not required. `_headers` is copied into the bundle for basic security and immutable asset caching headers.
