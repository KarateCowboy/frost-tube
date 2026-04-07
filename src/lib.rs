pub mod model;
pub mod services;

use iced::widget::{button, center, column, container, opaque, stack, text, text_input};
use iced::{Element, Task, Theme};
use rectum::InnerTubeClient;
use services::Video;

#[derive(Default, Debug)]
pub struct App {
    pub current_page: Page,
    pub search_query: String,
    pub search_results: Vec<Video>,
    pub error_message: Option<String>,
    pub client: InnerTubeClient,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchQueryChanged(String),
    Search,
    SearchResultsReceived(Result<Vec<Video>, String>),
    ViewVideo(String),
    DismissError,
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
                            .map(|results| {
                                results
                                    .items
                                    .into_iter()
                                    .filter_map(|item| match item {
                                        rectum::SearchItem::Video(v) => {
                                            Some(Video { title: v.title, id: v.id })
                                        }
                                    })
                                    .collect()
                            })
                            .map_err(|e| e.to_string())
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
            Message::ViewVideo(video_id) => {
                self.current_page = Page::VideoDetail { video_id };
                Task::none()
            }
            Message::DismissError => {
                self.error_message = None;
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Message> {
        let page = match self.current_page {
            Page::Index => column![
                text("Welcome to Frost Tube"),
                text_input("Search", &self.search_query).on_input(Message::SearchQueryChanged),
                button("Go").on_press(Message::Search)
            ]
            .padding(20)
            .into(),
            Page::SearchResults => {
                let mut col = column![text("Search Results")].padding(20);
                for video in &self.search_results {
                    col = col.push(
                        button(text(&video.title))
                            .on_press(Message::ViewVideo(video.id.clone())),
                    );
                }
                col.height(iced::Length::Fill)
                    .width(iced::Length::Fill)
                    .into()
            }
            Page::VideoDetail { ref video_id } => {
                let title = self
                    .search_results
                    .iter()
                    .find(|v| v.id == *video_id)
                    .map(|v| v.title.as_str())
                    .unwrap_or("Unknown Video");
                column![text(title)]
                    .padding(20)
                    .height(iced::Length::Fill)
                    .width(iced::Length::Fill)
                    .into()
            }
        };
        if let Some(err) = &self.error_message {
            stack![page, error_modal(err)].into()
        } else {
            page
        }
    }

    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn error_modal<'a>(error_message: &'a str) -> Element<'a, Message> {
    let alert = container(
        column![
            text(error_message),
            button(text("Dismiss")).on_press(Message::DismissError)
        ]
        .align_x(iced::Alignment::Center),
    )
    .width(iced::Length::Shrink)
    .style(|_theme| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb(
            0.2, 0.2, 0.2,
        ))),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: iced::Color::WHITE,
        },
        ..Default::default()
    });
    return opaque(
        container(alert)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgba(
                    0.0, 0.0, 0.0, 0.5,
                ))),
                ..Default::default()
            }),
    )
    .into();
}

#[derive(Debug, Default, PartialEq)]
pub enum Page {
    #[default]
    Index,
    SearchResults,
    VideoDetail { video_id: String },
}
