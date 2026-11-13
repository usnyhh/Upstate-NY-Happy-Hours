use iced::widget::{button, container, pick_list, row, space};
use iced::{Center, Element, Fill};

/// A top bar with a pick list on the left and action buttons on the right.
///
/// The pick list shows `options`, highlights `selected`, and emits
/// `on_select(option)` when the user picks one; `placeholder` is shown while
/// nothing is selected. `actions` are `(label, message)` pairs; each becomes a
/// button that emits its message when pressed.
pub fn navbar<'a, T, Message>(
    options: &'a [T],
    selected: Option<&'a T>,
    on_select: impl Fn(T) -> Message + 'a,
    placeholder: &'a str,
    actions: impl IntoIterator<Item = (&'a str, Message)>,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    Message: Clone + 'a,
{
    let picker = pick_list(options, selected, on_select)
        .placeholder(placeholder)
        .text_size(20);

    let buttons = row(actions
        .into_iter()
        .map(|(label, msg)| button(label).on_press(msg).into()))
    .spacing(8);

    container(row![picker, space::horizontal(), buttons].align_y(Center))
        .padding([8, 12])
        .width(Fill)
        .style(container::bordered_box)
        .into()
}
