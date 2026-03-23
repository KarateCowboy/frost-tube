pub mod model;

use iced::widget::{button, column, text, text_input};
use iced::{Element, Theme};

#[derive(Default, Debug)]
pub struct App {
    pub current_page: Page,
    pub search_query: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchQueryChanged(String),
    Search,
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
            }
            Message::Search => {
                self.current_page = Page::SearchResults;
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        column![
            text("Welcome to Frost Tube"),
            text_input("Search", &self.search_query)
                .on_input(Message::SearchQueryChanged),
            button("Go").on_press(Message::Search)
        ]
        .padding(20)
        .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Index,
    SearchResults,
}
