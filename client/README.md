# client

[Slint](https://slint.dev) frontend. The UI is declared in `ui/app.slint`
(compiled to Rust by `build.rs`); `src/app.rs` fills in its data and runs the
network requests. The same code builds for two targets:

- **web**: compiled to WebAssembly with [trunk](https://trunkrs.dev), drawn into
  a `<canvas>` with WebGL
- **desktop**: a native window (X11 or Wayland on Linux; OpenGL via femtovg,
  with a software fallback)

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
`src/api.rs`); start it with `nix run .#server`.

On launch the client looks up the user's approximate location from their public
IP (`https://ipinfo.io/json`, see `src/geolocate.rs`) and preselects the nearest
city, if one is within 150 km. Nothing is selected if the lookup fails, and a city
the user has already picked is never overridden.
