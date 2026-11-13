use std::sync::{Arc, Mutex};

use serde::Deserialize;

/// Base URL of the Axum server (see server/src/main.rs).
const API_BASE: &str = "http://127.0.0.1:3000/api";

/// Mirrors `entities::location::Model` on the server. Field names are the JSON keys,
/// so keep the two in sync.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[allow(dead_code)]
    pub location_id: i64,
    pub name: String,
    pub address: String,
}

/// State of a single in-flight or completed request.
enum Fetch<T> {
    Loading,
    Done(T),
    Failed(String),
}

type Shared<T> = Arc<Mutex<Fetch<T>>>;

pub struct App {
    greeting: Shared<String>,
    locations: Shared<Vec<Location>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self {
            greeting: Arc::new(Mutex::new(Fetch::Loading)),
            locations: Arc::new(Mutex::new(Fetch::Loading)),
        };
        app.fetch_all(&cc.egui_ctx);
        app
    }

    fn fetch_all(&self, ctx: &egui::Context) {
        fetch(ctx, &self.greeting, "/hello", |resp| {
            resp.text()
                .map(str::to_owned)
                .ok_or_else(|| "response was not UTF-8 text".to_owned())
        });
        fetch(ctx, &self.locations, "/locations", |resp| {
            resp.json::<Vec<Location>>().map_err(|e| e.to_string())
        });
    }
}

/// GET `API_BASE + path`, decode with `parse`, store into `slot`, and
/// repaint once the response arrives.
fn fetch<T: Send + 'static>(
    ctx: &egui::Context,
    slot: &Shared<T>,
    path: &str,
    parse: impl FnOnce(&ehttp::Response) -> Result<T, String> + Send + 'static,
) {
    *slot.lock().unwrap() = Fetch::Loading;

    let url = format!("{API_BASE}{path}");
    let slot = slot.clone();
    let ctx = ctx.clone();
    ehttp::fetch(ehttp::Request::get(&url), move |result| {
        let state = match result {
            Err(e) => Fetch::Failed(format!("Could not reach the API at {url}: {e}")),
            Ok(resp) if !resp.ok => Fetch::Failed(format!(
                "API request failed ({} {})",
                resp.status, resp.status_text
            )),
            Ok(resp) => match parse(&resp) {
                Ok(v) => Fetch::Done(v),
                Err(e) => Fetch::Failed(format!("Bad response from {url}: {e}")),
            },
        };
        *slot.lock().unwrap() = state;
        ctx.request_repaint();
    });
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        egui::Panel::top("navbar").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Upstate NY Happy Hours");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Refresh").clicked() {
                        self.fetch_all(&ctx);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            match &*self.greeting.lock().unwrap() {
                Fetch::Loading => {
                    ui.label("Contacting server…");
                }
                Fetch::Done(text) => {
                    ui.label(text);
                }
                Fetch::Failed(err) => {
                    ui.colored_label(ui.visuals().error_fg_color, err);
                }
            }

            ui.separator();
            ui.heading("Locations");

            match &*self.locations.lock().unwrap() {
                Fetch::Loading => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Loading locations…");
                    });
                }
                Fetch::Failed(err) => {
                    ui.colored_label(ui.visuals().error_fg_color, err);
                }
                Fetch::Done(locs) if locs.is_empty() => {
                    ui.label("No locations yet.");
                }
                Fetch::Done(locs) => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for loc in locs {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.strong(&loc.name);
                                ui.label(&loc.address);
                            });
                        }
                    });
                }
            }
        });
    }
}
