use frost_tube::Message;
use frost_tube::services::{Video, VideoService};
use iced::Element;
use iced_test::selector::{Candidate, Target};
use iced_test::Simulator;
use std::future::Future;
use std::pin::Pin;

pub fn find_button_by_text(ui: &mut Simulator<Message>, text: &str) -> Option<Target> {
    let text_target = ui.find(text).ok()?;
    let center = text_target.bounds().center();

    ui.find(|candidate: Candidate<'_>| match candidate {
        Candidate::Container {
            id,
            bounds,
            visible_bounds,
        } if bounds.contains(center) => Some(Target::Container {
            id: id.cloned(),
            bounds,
            visible_bounds,
        }),
        _ => None,
    })
    .ok()
}

#[derive(Debug)]
pub struct MockVideoService {
    pub results: Vec<Video>,
}

impl Default for MockVideoService {
    fn default() -> Self {
        Self {
            results: vec![
                Video {
                    title: "Kaze Fuiteru - Official Music Video".into(),
                    video_id: "abc123".into(),
                },
                Video {
                    title: "Kaze Fuiteru Live at Budokan".into(),
                    video_id: "def456".into(),
                },
            ],
        }
    }
}

impl VideoService for MockVideoService {
    fn search(&self, _query: &str) -> Pin<Box<dyn Future<Output = Vec<Video>> + Send + '_>> {
        let results = self.results.clone();
        Box::pin(async move { results })
    }
}
