mod app;

/// Native desktop entry point (`cargo run`).
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    const APP_NAME: &str = "Upstate NY Happy Hours";

    // Logs go to stderr; control verbosity with RUST_LOG (default: info).
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(APP_NAME)
            .with_app_id("upstate-ny-happy-hours")
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([360.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}

/// Web entry point (`trunk serve`): mounts onto `<canvas id="app_canvas">`.
#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast as _;

    // Route `log` output to the browser console.
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let canvas = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("app_canvas"))
        .expect("index.html must contain <canvas id=\"app_canvas\">")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("#app_canvas is not a <canvas>");

    wasm_bindgen_futures::spawn_local(async move {
        let result = eframe::WebRunner::new()
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await;

        if let Err(e) = result {
            log::error!("failed to start eframe: {e:?}");
        }
    });
}
