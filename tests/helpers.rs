use frost_tube::Message;
use frost_tube::services::{Video, VideoService};
use iced_test::selector::{Candidate, Target};
use iced_test::Simulator;

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
                    id: "abc123".into(),
                },
                Video {
                    title: "Kaze Fuiteru Live at Budokan".into(),
                    id: "def456".into(),
                },
            ],
        }
    }
}

impl VideoService for MockVideoService {
    async fn search(&self, _query: &str) -> Vec<Video> {
        self.results.clone()
    }
}
