use crate::services::Video;
use iced::widget::{button, column, container, opaque, stack, text, text_input};
use iced::{Element, Task, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    SearchQueryChanged(String),
    Search,
    SearchResultsReceived(Result<Vec<Video>, String>),
}

pub fn update(app: &mut crate::App, message: Message) -> Task<crate::Message> {
    match message {
        Message::SearchQueryChanged(query) => {
            app.search_query = query;
            Task::none()
        }
        Message::Search => {
            app.current_page = crate::Page::SearchResults;
            app.error_message = None;
            let client = app.client.clone();
            let query = app.search_query.clone();
            Task::perform(
                async move {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(client.search(&query))
                        .map(|results| {
                            results
                                .items
                                .into_iter()
                                .filter_map(|item| match item {
                                    rectum::SearchItem::Video(v) => Some(Video {
                                        title: v.title,
                                        id: v.id,
                                    }),
                                })
                                .collect()
                        })
                        .map_err(|e| e.to_string())
                },
                |r| crate::Message::Index(Message::SearchResultsReceived(r)),
            )
        }
        Message::SearchResultsReceived(result) => {
            match result {
                Ok(videos) => app.search_results = videos,
                Err(e) => app.error_message = Some(e),
            }
            Task::none()
        }
    }
}
pub fn view<'a>(app: &'a crate::App) -> Element<'a, crate::Message> {
    return column![
        text("Welcome to Frost Tube"),
        text_input("Search", app.search_query.as_str())
            .on_input(|s| crate::Message::Index(Message::SearchQueryChanged(s))),
        button("Go").on_press(crate::Message::Index(Message::Search))
    ]
    .padding(20)
    .into();
}
