{
  description = "BFCompiler - Brainfuck compiler toolchain written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        appDescription = "A Brainfuck compiler toolchain written in Rust.";

        bf = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;

          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

          doCheck = false;

          meta = with pkgs.lib; {
            description = appDescription;
            homepage = "https://github.com/4ster-light/bfcompiler";
            license = licenses.mit;
            mainProgram = manifest.name;
          };
        };
      in
      {
        packages = {
          inherit bf;
          default = bf;
        };

        checks = {
          package = bf;

          smoke =
            pkgs.runCommand "bf-smoke"
              {
                nativeBuildInputs = [ bf ];
              }
              ''
                ${pkgs.lib.getExe bf} --version
                mkdir -p $out
              '';
        };

        apps.default = {
          type = "app";
          program = pkgs.lib.getExe bf;
          meta.description = appDescription;
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = [ bf ];
          packages = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            git
          ];
        };
      }
    );
}
