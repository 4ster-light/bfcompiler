{
  description = "BFCompiler - Brainfuck compiler toolchain written in Rust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        appDescription = "A Brainfuck compiler toolchain written in Rust.";
        bf = pkgs.rustPlatform.buildRustPackage {
          pname = "bf";
          version = "1.0.0";
          src = builtins.path { path = ./.; };
          cargoLock = { lockFile = ./Cargo.lock; };
          doCheck = false;
          meta = with pkgs.lib; {
            description = appDescription;
            homepage = "https://github.com/4ster-light/bfcompiler";
            license = licenses.mit;
            mainProgram = "bf";
          };
        };
      in
      {
        packages.default = bf;
        packages.bf = bf;

        apps.default = {
          type = "app";
          program = "${bf}/bin/bf";
          description = appDescription;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [ cargo rustc rustfmt clippy ];
        };
      }
    );
}
