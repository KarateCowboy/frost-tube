pub mod invidious;
pub mod model;
pub mod services;

use iced::widget::{button, column, text, text_input};
use iced::{Element, Task, Theme};
use invidious::InvidiousClient;
use services::{Video, VideoService};

#[derive(Default, Debug)]
pub struct App {
    pub current_page: Page,
    pub search_query: String,
    pub search_results: Vec<Video>,
    pub error_message: Option<String>,
    pub client: InvidiousClient,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchQueryChanged(String),
    Search,
    SearchResultsReceived(Result<Vec<Video>, String>),
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SearchQueryChanged(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::Search => {
                self.current_page = Page::SearchResults;
                self.error_message = None;
                let client = self.client.clone();
                let query = self.search_query.clone();
                Task::perform(
                    async move {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(client.search(&query))
                    },
                    Message::SearchResultsReceived,
                )
            }
            Message::SearchResultsReceived(result) => {
                match result {
                    Ok(videos) => self.search_results = videos,
                    Err(e) => self.error_message = Some(e),
                }
                Task::none()
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
                if let Some(err) = &self.error_message {
                    col = col.push(text(format!("Error: {err}")));
                }
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
