pub mod invidious;
pub mod model;
pub mod services;

use iced::widget::{button, column, text, text_input};
use iced::{Element, Theme};
use services::Video;

#[derive(Default, Debug)]
pub struct App {
    pub current_page: Page,
    pub search_query: String,
    pub search_results: Vec<Video>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchQueryChanged(String),
    Search,
    SearchResultsReceived(Vec<Video>),
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
            Message::SearchResultsReceived(results) => {
                self.search_results = results;
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        match self.current_page {
            Page::Index => {
                column![
                    text("Welcome to Frost Tube"),
                    text_input("Search", &self.search_query)
                        .on_input(Message::SearchQueryChanged),
                    button("Go").on_press(Message::Search)
                ]
                .padding(20)
                .into()
            }
            Page::SearchResults => {
                let mut col = column![text("Search Results")].padding(20);
                for video in &self.search_results {
                    col = col.push(text(&video.title));
                }
                col.into()
            }
        }
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
