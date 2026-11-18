mod api;
mod app;
mod city;
mod geolocate;
mod http;

use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    init_logging();

    // Wayland app id / X11 WM_CLASS; must be set before the window is created.
    #[cfg(target_os = "linux")]
    slint::set_xdg_app_id("upstate-ny-happy-hours")?;

    // On the web, Slint renders into `<canvas id="canvas">` in index.html.
    app::new()?.run()
}

/// Native: logs go to stderr; control verbosity with RUST_LOG (default: info).
#[cfg(not(target_arch = "wasm32"))]
fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

/// Web: route `log` output and panics to the browser console.
#[cfg(target_arch = "wasm32")]
fn init_logging() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).ok();
}
