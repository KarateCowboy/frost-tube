use iced::widget::{column, text};
use iced::{Element, Theme};

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Frost Tube")
        .theme(App::theme)
        .run()
}

#[derive(Default)]
struct App {
    // application state goes here
}

#[derive(Debug, Clone)]
enum Message {
    // define your messages here
}

impl App {
    fn update(&mut self, _message: Message) {
        // handle messages here
    }

    fn view(&self) -> Element<Message> {
        column![text("Welcome to Frost Tube")]
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
