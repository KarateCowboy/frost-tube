use frost_tube::Message;
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
