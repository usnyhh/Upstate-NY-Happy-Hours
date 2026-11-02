use iced::widget::{column, container, text};
use iced::{Center, Element, Length, Theme};

mod components;
use components::navbar::Navbar;

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .default_font(iced::Font::with_name("Fira Sans"))
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavTab {
    Home,
    Settings,
    About,
}


struct App {
    current_tab: NavTab
}

impl Default for App {
    fn default() -> Self {
        Self { current_tab: NavTab::Home }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TabSelected(NavTab),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::TabSelected(tab) => {
                self.current_tab = tab;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
                // 2. Simply call your modular component view function here
        let navbar = Navbar::view(self.current_tab);

        let content_text = match self.current_tab {
            NavTab::Home => "Welcome to the Home Screen!",
            NavTab::Settings => "Adjust your Settings here.",
            NavTab::About => "Learn more About this application.",
        };

        let content_area = container(text(content_text).size(24))
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Center)
            .align_y(Center);

        column![navbar, content_area]
            .spacing(20)
            .padding(20)
            .into()
    }
}
