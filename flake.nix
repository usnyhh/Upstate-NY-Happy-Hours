{
  description = "Upstate NY Happy Hours: egui (wasm) client + axum server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        lib = pkgs.lib;

        # ---- server: a normal Nix package ----------------------------------
        server = pkgs.rustPlatform.buildRustPackage {
          pname = "server";
          version = "0.1.0";
          src = lib.fileset.toSource {
            root = ./server;
            fileset = lib.fileset.unions [
              ./server/src
              ./server/Cargo.toml
              ./server/Cargo.lock
            ];
          };
          cargoLock.lockFile = ./server/Cargo.lock;
          # sqlx links against the system libsqlite3.
          buildInputs = [ pkgs.sqlite ];
        };

        # ---- client: egui app built to wasm with trunk ----------------------
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # trunk fetches the wasm-bindgen CLI matching Cargo.lock on first
        # build; binaryen provides wasm-opt for release builds.
        # nixpkgs' trunk currently fails to build: zstd-sys can't find libzstd.
        trunk = pkgs.trunk.overrideAttrs (old: {
          buildInputs = (old.buildInputs or [ ]) ++ [ pkgs.zstd ];
        });
        clientTools = [ rustToolchain trunk pkgs.binaryen ];

        # trunk needs network access (crates.io), so the client is built in
        # the working tree rather than in the Nix sandbox.
        findRoot = ''
          root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
          if [ ! -f "$root/client/Trunk.toml" ]; then
            echo "error: run this from inside the Upstate-NY-Happy-Hours repo" >&2
            exit 1
          fi
        '';

        client-build = pkgs.writeShellApplication {
          name = "client-build";
          runtimeInputs = clientTools ++ [ pkgs.git ];
          text = ''
            ${findRoot}
            cd "$root/client"
            trunk build --release
          '';
        };

        client-serve = pkgs.writeShellApplication {
          name = "client-serve";
          runtimeInputs = clientTools ++ [ pkgs.git ];
          text = ''
            ${findRoot}
            cd "$root/client"
            exec trunk serve --port "''${CLIENT_PORT:-8080}"
          '';
        };

        # ---- both together --------------------------------------------------
        run-all = pkgs.writeShellApplication {
          name = "run-all";
          runtimeInputs = [ server client-serve ];
          text = ''
            server &
            server_pid=$!
            trap 'kill "$server_pid" 2>/dev/null || true' EXIT INT TERM
            client-serve
          '';
        };

        mkApp = drv: name: {
          type = "app";
          program = "${drv}/bin/${name}";
        };

        serverShell = pkgs.mkShell {
          packages = [ rustToolchain pkgs.sqlite ];
        };

        clientShell = pkgs.mkShell {
          packages = clientTools;
        };
      in
      {
        packages = {
          default = server;
          inherit server client-build client-serve run-all;
        };

        apps = {
          default = mkApp run-all "run-all";
          server = mkApp server "server";
          client = mkApp client-serve "client-serve";
          client-build = mkApp client-build "client-build";
        };

        devShells = {
          default = serverShell;
          server = serverShell;
          client = clientShell;
        };
      });
}
