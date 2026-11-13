mod api;
mod app;
mod city;
mod components;
mod geolocate;
mod http;

const APP_NAME: &str = "Upstate NY Happy Hours";

fn main() -> iced::Result {
    init_logging();

    iced::application(app::App::new, app::App::update, app::App::view)
        .title(APP_NAME)
        .theme(iced::Theme::Dark)
        .window(window_settings())
        .run()
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

fn window_settings() -> iced::window::Settings {
    iced::window::Settings {
        size: iced::Size::new(900.0, 650.0),
        min_size: Some(iced::Size::new(360.0, 300.0)),
        // On the web, iced replaces `<div id="iced">` in index.html (the default
        // target), so nothing platform-specific is needed there.
        #[cfg(target_os = "linux")]
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "upstate-ny-happy-hours".into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
