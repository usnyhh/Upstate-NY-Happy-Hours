//! Glue between the Slint UI (`ui/app.slint`) and the rest of the client: fills
//! in the UI's properties and runs the network requests it triggers.

use std::future::Future;
use std::rc::Rc;

use slint::{ComponentHandle, ModelRc, SharedString, VecModel, Weak};

use crate::api::{self, Location};
use crate::city::City;
use crate::geolocate::{self, Coords};

slint::include_modules!();

/// Build the window, wire up its callbacks, and start the startup requests.
pub fn new() -> Result<AppWindow, slint::PlatformError> {
    let ui = AppWindow::new()?;

    let cities: Vec<SharedString> = City::ALL.iter().map(|c| c.name().into()).collect();
    ui.set_cities(ModelRc::from(Rc::new(VecModel::from(cities))));

    let weak = ui.as_weak();
    ui.on_refresh(move || load_locations(weak.clone()));

    load_locations(ui.as_weak());
    let weak = ui.as_weak();
    spawn(async move {
        let result = geolocate::locate().await;
        if let Some(ui) = weak.upgrade() {
            on_located(&ui, result);
        }
    });

    Ok(ui)
}

/// The located city is only a default: never override one the user already
/// picked, and fail quietly since the picker still works without it.
fn on_located(ui: &AppWindow, result: Result<Coords, String>) {
    match result {
        Ok(here) if ui.get_selected_city() < 0 => {
            if let Some(city) = City::nearest(here) {
                let index = City::ALL.iter().position(|&c| c == city).expect("city in ALL");
                ui.set_selected_city(index as i32);
            }
        }
        Ok(_) => {}
        Err(err) => log::warn!("{err}"),
    }
}

/// Show the loading state, fetch the locations, then show the result.
fn load_locations(weak: Weak<AppWindow>) {
    if let Some(ui) = weak.upgrade() {
        ui.set_locations_state(FetchState::Loading);
    }
    spawn(async move {
        let result = api::fetch_locations().await;
        let Some(ui) = weak.upgrade() else { return };
        match result {
            Ok(locations) => {
                let items: Vec<LocationItem> = locations.iter().map(to_item).collect();
                ui.set_locations(ModelRc::from(Rc::new(VecModel::from(items))));
                ui.set_locations_state(FetchState::Done);
            }
            Err(err) => {
                ui.set_locations_error(err.into());
                ui.set_locations_state(FetchState::Failed);
            }
        }
    });
}

fn to_item(location: &Location) -> LocationItem {
    LocationItem {
        name: location.name.as_str().into(),
        address: location.address.as_str().into(),
    }
}

/// Run `future` on Slint's event loop, so it can touch the UI when it finishes.
fn spawn(future: impl Future<Output = ()> + 'static) {
    // reqwest needs a tokio context on native; the browser drives it on wasm.
    #[cfg(not(target_arch = "wasm32"))]
    let future = async_compat::Compat::new(future);

    if let Err(err) = slint::spawn_local(future) {
        log::error!("could not start background task: {err}");
    }
}
