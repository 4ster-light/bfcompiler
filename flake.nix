{
  description = "BFCompiler - a small Brainfuck compiler, interpreter, REPL, and web playground";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        inherit (pkgs) lib;

        cargoToml = pkgs.lib.importTOML ./Cargo.toml;
        manifest = cargoToml.package;
        packageVersion =
          if builtins.isString manifest.version then
            manifest.version
          else
            cargoToml.workspace.package.version;
        cliDescription = "A Brainfuck compiler, interpreter, and REPL written in Rust.";
        webDescription = "A local-first Brainfuck playground built with Rust, WebAssembly, and Vite.";

        # Pin the pnpm major used by both the dependency fetch and the build.
        pnpm = pkgs.pnpm_12;

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rustfmt"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Keep generated dependencies and build outputs out of the Nix source.
        cleanGenerated =
          path: _type:
          !(builtins.elem (baseNameOf (toString path)) [
            "node_modules"
            "dist"
            "target"
            "test-results"
            "playwright-report"
            "wasm"
            ".direnv"
          ]);
        source = pkgs.lib.cleanSourceWith {
          src = pkgs.lib.cleanSource ./.;
          filter = cleanGenerated;
        };

        # fetchPnpmDeps only needs the manifests; keeping the source this small
        # means editing CSS or docs does not invalidate the dependency fetch.
        pnpmSource = pkgs.lib.cleanSourceWith {
          src = ./web;
          filter =
            path: type:
            cleanGenerated path type
            && (
              type == "directory"
              || (
                type == "regular"
                && builtins.elem (baseNameOf (toString path)) [
                  "package.json"
                  "pnpm-lock.yaml"
                  "pnpm-workspace.yaml"
                ]
              )
            );
        };

        commonArgs = {
          src = source;
          strictDeps = true;
          nativeBuildInputs = [ pkgs.lld ];
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        bf = craneLib.buildPackage (
          commonArgs
          // {
            pname = manifest.name;
            version = packageVersion;
            inherit cargoArtifacts;
            doCheck = false;

            meta = with lib; {
              description = cliDescription;
              homepage = "https://github.com/4ster-light/bfcompiler";
              license = licenses.mit;
              mainProgram = manifest.name;
            };
          }
        );

        # Internal build step: compile the browser bindings and generate the
        # wasm-bindgen glue. This is copied into the web build, not exported as
        # a separate flake package.
        wasmArgs = commonArgs // {
          pname = "bf-web-wasm";
          version = packageVersion;
          cargoExtraArgs = "--locked --package bf-web --target wasm32-unknown-unknown";
          doCheck = false;
        };

        web-wasm = craneLib.buildPackage (
          wasmArgs
          // {
            cargoArtifacts = craneLib.buildDepsOnly wasmArgs;
            nativeBuildInputs = wasmArgs.nativeBuildInputs ++ [ pkgs.wasm-bindgen-cli_0_2_129 ];
            doCheck = false;

            installPhase = ''
              runHook preInstall
              mkdir -p "$out"
              wasm-bindgen \
                --target web \
                --out-dir "$out" \
                --out-name bf_web \
                target/wasm32-unknown-unknown/release/bf_web.wasm
              runHook postInstall
            '';
          }
        );

        # Hermetic pnpm dependency cache. Update this hash only when
        # web/pnpm-lock.yaml or the manifest files change.
        pnpmDeps = pkgs.fetchPnpmDeps {
          pname = "bf-web";
          version = packageVersion;
          src = pnpmSource;
          inherit pnpm;
          fetcherVersion = 4;
          hash = "sha256-vPmgHBJF113VXLiuoUDPRKjjMRbv7fD6uIKCtpf3XKY=";
        };

        web = pkgs.stdenv.mkDerivation {
          pname = "bf-web";
          version = packageVersion;

          src = source;
          nativeBuildInputs = [
            pkgs.nodejs_26
            pnpm
            pkgs.pnpmConfigHook
          ];

          # The pnpm project lives in web/, not at the repository root.
          pnpmRoot = "web";
          inherit pnpmDeps;

          buildPhase = ''
            runHook preBuild

            rm -rf web/src/wasm
            mkdir -p web/src/wasm
            cp -R ${web-wasm}/. web/src/wasm/

            (
              cd web
              pnpm run build
            )

            runHook postBuild
          '';

          installPhase = ''
            runHook preInstall
            mkdir -p "$out"
            cp -R web/dist/. "$out/"
            runHook postInstall
          '';

          meta = with lib; {
            description = webDescription;
            homepage = "https://github.com/4ster-light/bfcompiler";
            license = licenses.mit;
          };
        };

        webApp = {
          type = "app";
          program = lib.getExe (
            pkgs.writeShellApplication {
              name = "bf-web";
              runtimeInputs = [ pkgs.static-server ];
              text = ''
                port="''${PORT:-4173}"
                exec static-server -host 0.0.0.0 -port "$port" ${web}
              '';
            }
          );
          meta.description = "Serve the built web playground on port 4173";
        };
      in
      {
        packages = {
          default = bf;
          inherit bf web;
        };

        checks = {
          inherit bf web;

          rustfmt = craneLib.cargoFmt { inherit (commonArgs) src; };

          clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--workspace --all-targets -- -D warnings";
            }
          );

          tests = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--workspace";
            }
          );

          nixfmt = pkgs.runCommand "nixfmt-check" { nativeBuildInputs = [ pkgs.nixfmt ]; } ''
            nixfmt --check ${./flake.nix}
            touch "$out"
          '';

          web-format =
            pkgs.runCommand "web-format"
              {
                nativeBuildInputs = [ pkgs.biome ];
              }
              ''
                cd ${source}/web
                biome check --formatter-enabled=true --linter-enabled=false --assist-enabled=false .
                touch "$out"
              '';

          smoke = pkgs.runCommand "bf-smoke" { nativeBuildInputs = [ bf ]; } ''
            ${lib.getExe bf} --version
            mkdir -p "$out"
          '';
        };

        apps.default = {
          type = "app";
          program = lib.getExe bf;
          meta.description = cliDescription;
        };

        apps.web = webApp;

        formatter = pkgs.nixfmt-tree;

        devShells.default = craneLib.devShell {
          packages = [
            rustToolchain
            pkgs.nodejs_26
            pnpm
            pkgs.wasm-pack
            pkgs.wasm-bindgen-cli_0_2_129
            pkgs.lld
            pkgs.biome
          ];

          shellHook = ''
            cat <<'EOF'
            BFCompiler development environment

              Format:
                cargo fmt --all
                nix fmt
                cd web && pnpm fmt

              Check:
                cargo clippy --workspace --all-targets -- -D warnings
                cargo test --workspace
                nix flake check

              Web playground:
                cd web
                pnpm install
                pnpm run          # discover scripts

              Nix:
                nix build .#web
                nix run .#web     # serve the built bundle on :4173
            EOF'';
        };
      }
    );
}
