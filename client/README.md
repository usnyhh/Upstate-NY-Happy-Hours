# client

egui frontend, compiled to WebAssembly with [trunk](https://trunkrs.dev).

```sh
nix develop ..#client   # rust (with wasm32 target), trunk, binaryen
trunk serve             # dev server on http://127.0.0.1:8080, rebuilds on change
trunk build --release   # optimized build into ./dist
```

Or from anywhere in the repo: `nix run .#client` (serve) / `nix run .#client-build`.

The app talks to the server at `http://127.0.0.1:3000/api` (`API_BASE` in
`src/app.rs`); start it with `nix run .#server`, or both with `nix run`.
