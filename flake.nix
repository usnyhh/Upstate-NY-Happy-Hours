{
  description = "Upstate NY Happy Hours: miso client + servant server";

  # Miso's cachix cache, so that the client dev shells (e.g. haskell-language-server
  # in .#hls) are downloaded prebuilt instead of built from source. A flake's
  # nixConfig only applies to the flake you run, not to its inputs, so this
  # repeats miso's.
  nixConfig = {
    extra-substituters = [
      "https://haskell-miso-cachix.cachix.org"
    ];
    extra-trusted-public-keys = [
      "haskell-miso-cachix.cachix.org-1:m8hN1cvFMJtYib4tj+06xkKt5ABMSGfe8W7s40x1kQ0="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    miso.url = "github:dmjio/miso";
    flake-utils.follows = "miso/flake-utils";
  };

  outputs = { self, nixpkgs, miso, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        lib = pkgs.lib;
        misoShells = miso.outputs.devShells.${system};

        # Toolchain from miso's wasm shell (wasm32-wasi-ghc, cabal, make,
        # wasm-opt, http-server, ...), reused by the client run script.
        wasmTools = misoShells.wasm.nativeBuildInputs ++ misoShells.wasm.buildInputs;

        # ---- server: a normal Nix package ----------------------------------
        server = pkgs.haskellPackages.callCabal2nix "server"
          (lib.fileset.toSource {
            root = ./server;
            fileset = lib.fileset.unions [ ./server/app ./server/server.cabal ];
          })
          { };

        # ---- client: built in the working tree --------------------------------
        # The wasm build needs network access (cabal fetches Hackage and the
        # miso git dependency), so it can't run in the Nix sandbox. Instead
        # these scripts run the Makefile in ./client with miso's toolchain.
        findRoot = ''
          root="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
          if [ ! -f "$root/client/Makefile" ]; then
            echo "error: run this from inside the Upstate-NY-Happy-Hours repo" >&2
            exit 1
          fi
        '';

        client-build = pkgs.writeShellApplication {
          name = "client-build";
          runtimeInputs = wasmTools ++ [ pkgs.git ];
          text = ''
            ${findRoot}
            make -C "$root/client" all
          '';
        };

        client-serve = pkgs.writeShellApplication {
          name = "client-serve";
          runtimeInputs = wasmTools ++ [ pkgs.git client-build ];
          text = ''
            ${findRoot}
            client-build
            exec http-server "$root/client/public" -p "''${CLIENT_PORT:-8080}"
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
          packages = [
            pkgs.ghc
            pkgs.cabal-install
            pkgs.zlib
            pkgs.pkg-config
          ];
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

          # Client (miso) shells
          client = misoShells.default;
          wasm = misoShells.wasm;
          ghcjs = misoShells.ghcjs;
          mhs = misoShells.mhs;
          hls = misoShells.hls;
        };
      });
}
