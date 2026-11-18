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
    let ui = app::new()?;
    #[cfg(target_arch = "wasm32")]
    fill_browser_window(&ui);
    ui.run()
}

/// Web: Slint would size the canvas to the window's preferred size (900x650 in
/// ui/app.slint), so instead size it to the browser window, now and whenever
/// that resizes. Slint's logical pixels are CSS pixels on the web.
#[cfg(target_arch = "wasm32")]
fn fill_browser_window(ui: &app::AppWindow) {
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let Some(browser) = web_sys::window() else { return };

    let weak = ui.as_weak();
    let fit = move || {
        let (Some(ui), Some(browser)) = (weak.upgrade(), web_sys::window()) else { return };
        let px = |v: Result<wasm_bindgen::JsValue, _>| v.ok().and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
        let size = slint::LogicalSize::new(px(browser.inner_width()), px(browser.inner_height()));
        if size.width > 0.0 && size.height > 0.0 {
            ui.window().set_size(size);
        }
    };
    fit();

    // Lives as long as the page, so leaking it is fine.
    let on_resize = Closure::<dyn FnMut()>::new(fit);
    if let Err(err) = browser.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref()) {
        log::warn!("could not watch browser window resizes: {err:?}");
    }
    on_resize.forget();
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
