# Native GHC + cabal for the server: `nix-shell --run 'cabal run server'`
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  packages = [
    pkgs.ghc
    pkgs.cabal-install
    pkgs.zlib
    pkgs.pkg-config
  ];
}
