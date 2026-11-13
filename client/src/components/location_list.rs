use iced::widget::{column, container, scrollable, text};
use iced::{Element, Fill, Font, font};

use crate::api::Location;

/// A scrollable column of location cards that fills the available height.
pub fn location_list<'a, Message: 'a>(locations: &'a [Location]) -> Element<'a, Message> {
    let cards = column(locations.iter().map(location_card))
        .spacing(8)
        // Keep cards clear of the scrollbar.
        .padding(iced::Padding::ZERO.right(12));

    scrollable(cards).height(Fill).into()
}

/// One bordered card showing a location's name (bold) and address.
fn location_card<'a, Message: 'a>(location: &'a Location) -> Element<'a, Message> {
    let bold = Font {
        weight: font::Weight::Bold,
        ..Font::DEFAULT
    };

    container(column![text(&location.name).font(bold), text(&location.address)].spacing(4))
        .padding(10)
        .width(Fill)
        .style(container::bordered_box)
        .into()
}
