//! Top-level iced application: state, messages, update, and view.

use iced::widget::{column, text};
use iced::{Element, Task};

use crate::api::{self, Location};
use crate::city::City;
use crate::components::{location_list, navbar};
use crate::geolocate::{self, Coords};

#[derive(Debug, Clone)]
pub enum Message {
    Refresh,
    CitySelected(City),
    Located(Result<Coords, String>),
    LocationsLoaded(Result<Vec<Location>, String>),
}

#[derive(Default)]
pub struct App {
    city: Option<City>,
    locations: Fetch<Vec<Location>>,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let startup = Task::batch([
            load_locations(),
            Task::perform(geolocate::locate(), Message::Located),
        ]);
        (Self::default(), startup)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Refresh => {
                self.locations = Fetch::Loading;
                return load_locations();
            }
            Message::CitySelected(city) => self.city = Some(city),
            Message::Located(result) => self.on_located(result),
            Message::LocationsLoaded(result) => self.locations = result.into(),
        }
        Task::none()
    }

    /// The located city is only a default: never override one the user already
    /// picked, and fail quietly since the picker still works without it.
    fn on_located(&mut self, result: Result<Coords, String>) {
        match result {
            Ok(here) if self.city.is_none() => self.city = City::nearest(here),
            Ok(_) => {}
            Err(err) => log::warn!("{err}"),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let navbar = navbar(
            &City::ALL,
            self.city.as_ref(),
            Message::CitySelected,
            "Choose a city",
            [("Refresh", Message::Refresh)],
        );

        let body = column![text("Locations").size(20), self.locations_view()]
            .spacing(10)
            .padding(12);

        column![navbar, body].into()
    }

    fn locations_view(&self) -> Element<'_, Message> {
        match &self.locations {
            Fetch::Loading => text("Loading locations…").into(),
            Fetch::Failed(err) => text(err).style(text::danger).into(),
            Fetch::Done(locations) if locations.is_empty() => text("No locations yet.").into(),
            Fetch::Done(locations) => location_list(locations),
        }
    }
}

fn load_locations() -> Task<Message> {
    Task::perform(api::fetch_locations(), Message::LocationsLoaded)
}

/// State of a single in-flight or completed request.
#[derive(Debug, Default)]
enum Fetch<T> {
    #[default]
    Loading,
    Done(T),
    Failed(String),
}

impl<T> From<Result<T, String>> for Fetch<T> {
    fn from(result: Result<T, String>) -> Self {
        match result {
            Ok(v) => Fetch::Done(v),
            Err(e) => Fetch::Failed(e),
        }
    }
}
