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

<https://bfcompiler.pages.dev> is deployed by [`.github/workflows/pages.yml`](../.github/workflows/pages.yml) on every push to `main` that touches the playground or its Rust dependencies, and on manual `workflow_dispatch`. `wrangler.jsonc` pins the project name and the build output directory, so a deploy run from `web/` takes no flags. Wrangler comes from the dev shell; authentication comes from `wrangler login` or `CLOUDFLARE_API_TOKEN`:

```bash
cd web
nix develop -c wrangler pages deploy --branch=main
```

Add `--commit-dirty=true` when the working tree has uncommitted changes, otherwise wrangler only warns. CI builds the bundle with the non-Nix commands (`pnpm install --frozen-lockfile && pnpm wasm && pnpm build`) and then deploys `dist/`. It needs these repository secrets:

| Secret | Purpose |
| --- | --- |
| `CLOUDFLARE_API_TOKEN` | API token with the *Cloudflare Pages: Edit* permission |
| `CLOUDFLARE_ACCOUNT_ID` | Account id printed by `nix develop -c wrangler whoami` |

Create the token at <https://dash.cloudflare.com/profile/api-tokens> with *Create Token* → *Create Custom Token*: permission **Account → Cloudflare Pages → Edit**, account resources limited to your own account, then store it as the `CLOUDFLARE_API_TOKEN` secret.

`_headers` is copied into the bundle for basic security and immutable asset caching headers. The bundle is fully static: no Pages Functions and no server-side rendering.

Project names are global across all Cloudflare accounts and the `*.pages.dev` subdomain is the project name, so a taken name gets a random suffix. `bfcompiler` was free; `brainfuck.pages.dev` and `bf.pages.dev` belong to other accounts.

To create a *new* Pages project, run `nix develop -c wrangler pages project create <name> --production-branch main --force`. That is the only place `--force` is needed: without it wrangler delegates to the Workers-based Pages flow, which rejects a Pages-style `wrangler.jsonc`. Once the project exists, deploys go straight to Cloudflare Pages and must omit `--force`.
