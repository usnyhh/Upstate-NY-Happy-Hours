use iced::widget::{button, column, text, center};
use iced::{Element, Length, Theme};

pub fn main() -> iced::Result {
    iced::application(Counter::default, Counter::update, Counter::view)
        .default_font(iced::Font::with_name("Fira Sans"))
        .run()
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text(format!("Count: {}", self.value))
            .size(50)
            .color(Theme::Dark.palette().text);

        let content = column![
            button("Increment").on_press(Message::Increment).padding(10),
            title,
            button("Decrement").on_press(Message::Decrement).padding(10),
        ]
        .spacing(20)
        .align_x(iced::Alignment::Center);

        // Center container forces layout calculations to scale out across the browser window
        center(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
}
