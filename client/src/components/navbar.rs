use iced::widget::{button, row, text};
use iced::{Element, Length};
// Assuming NavTab and Message live in a shared place or main
use crate::{Message, NavTab}; 

/// The Navbar component structure. 
/// In Iced, for pure layout pieces, a helper struct or a function works beautifully.
pub struct Navbar;

impl Navbar {
    pub fn view(active_tab: NavTab) -> Element<'static, Message> {
        // A helper closure to style buttons conditionally based on the active tab
        let make_button = |tab: NavTab, label: &'static str| {
            let mut btn = button(text(label).size(16))
                .on_press(Message::TabSelected(tab))
                .padding(10);
            
            // Optional visual feedback: use a different style if it's the current tab
            if tab == active_tab {
                btn = btn.style(button::success); // Highlights the active tab
            }

            btn
        };

        row![
            make_button(NavTab::Home, "Home"),
            make_button(NavTab::Settings, "Settings"),
            make_button(NavTab::About, "About"),
        ]
        .spacing(10)
        .width(Length::Fill)
        .into()
    }
}
