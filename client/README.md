# client

egui frontend. The same `src/app.rs` builds for two targets:

- **web**: compiled to WebAssembly with [trunk](https://trunkrs.dev)
- **desktop**: a native window (X11 or Wayland on Linux)

```sh
nix develop ..#client   # rust (with wasm32 target), trunk, binaryen, desktop GL/windowing libs

# web
trunk serve             # dev server on http://127.0.0.1:8080, rebuilds on change
trunk build --release   # optimized build into ./dist

# desktop
cargo run               # native window; RUST_LOG=debug for more logging
cargo run --release
```

Or from anywhere in the repo:

| command                  | what                                  |
|--------------------------|---------------------------------------|
| `nix run .#client`       | web dev server                        |
| `nix run .#client-build` | web release build into `client/dist`  |
| `nix run .#desktop`      | desktop app (sandboxed Nix build)     |
| `nix run`                | server + web client                   |
| `nix run .#run-desktop`  | server + desktop client               |

The app talks to the server at `http://127.0.0.1:3000/api` (`API_BASE` in
`src/app.rs`); start it with `nix run .#server`.
