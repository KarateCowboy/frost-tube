pub mod model;

use iced::widget::{column, text};
use iced::{Element, Theme};

#[derive(Default, Debug)]
pub struct App {
    pub current_page: Page, // application state goes here
}

#[derive(Debug, Clone)]
pub enum Message {
    // define your messages here
}

impl App {
    pub fn update(&mut self, _message: Message) {
        // handle messages here
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        column![text("Welcome to Frost Tube")].padding(20).into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Index,
}
