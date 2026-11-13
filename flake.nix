{
  description = "Upstate NY Happy Hours: iced client (web + desktop) + axum server";

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

        # ---- client: iced app built to wasm with trunk ----------------------
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

        # ---- client: native desktop build -----------------------------------
        # winit and wgpu dlopen these at runtime rather than linking them, so the
        # binary has to be told where they live. wgpu prefers Vulkan and falls
        # back to GL; iced falls back to its tiny-skia software renderer.
        desktopRuntimeLibs = with pkgs; [
          vulkan-loader
          libGL
          libxkbcommon
          wayland
          libx11
          libxcursor
          libxi
          libxrandr
        ];

        # Unlike the wasm build this needs no network (cargoLock vendors the
        # crates), so it is a normal sandboxed package.
        client-desktop = pkgs.rustPlatform.buildRustPackage {
          pname = "client-desktop";
          version = "0.1.0";
          src = lib.fileset.toSource {
            root = ./client;
            fileset = lib.fileset.unions [
              ./client/src
              ./client/Cargo.toml
              ./client/Cargo.lock
            ];
          };
          cargoLock.lockFile = ./client/Cargo.lock;
          nativeBuildInputs = [ pkgs.makeWrapper ];
          # reqwest's rustls backend (for the HTTPS IP-geolocation lookup) uses
          # aws-lc-sys. nixpkgs sets AWS_LC_SYS_USE_SYSTEM=1, but its aws-lc isn't
          # one aws-lc-sys accepts, so build the vendored copy as `cargo run` does.
          preBuild = ''
            unset AWS_LC_SYS_USE_SYSTEM
          '';
          postInstall = ''
            mv $out/bin/client $out/bin/client-desktop
            wrapProgram $out/bin/client-desktop \
              --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath desktopRuntimeLibs}
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

        run-desktop = pkgs.writeShellApplication {
          name = "run-desktop";
          runtimeInputs = [ server client-desktop ];
          text = ''
            server &
            server_pid=$!
            trap 'kill "$server_pid" 2>/dev/null || true' EXIT INT TERM
            client-desktop
          '';
        };

        mkApp = drv: name: {
          type = "app";
          program = "${drv}/bin/${name}";
        };

        serverShell = pkgs.mkShell {
          packages = [ rustToolchain pkgs.sqlite ];
        };

        # Covers both targets: `trunk serve` for web, `cargo run` for desktop.
        clientShell = pkgs.mkShell {
          packages = clientTools;
          LD_LIBRARY_PATH = lib.makeLibraryPath desktopRuntimeLibs;
        };
      in
      {
        packages = {
          default = server;
          inherit server client-build client-serve client-desktop run-all run-desktop;
        };

        apps = {
          default = mkApp run-all "run-all";
          server = mkApp server "server";
          client = mkApp client-serve "client-serve";
          client-build = mkApp client-build "client-build";
          desktop = mkApp client-desktop "client-desktop";
          run-desktop = mkApp run-desktop "run-desktop";
        };

        devShells = {
          default = serverShell;
          server = serverShell;
          client = clientShell;
        };
      });
}
