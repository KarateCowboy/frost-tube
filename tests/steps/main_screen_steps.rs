use crate::FrostTubeWorld;
use crate::helpers;

use cucumber::{given, then, when};
use frost_tube::*;
use iced_test::selector::Text;
use iced_test::simulator;

#[given("I have a new application instance")]
fn i_have_a_new_application_instance(world: &mut FrostTubeWorld) {
    world.app = App::default();
}
#[given(expr = "I am on the {string} page")]
fn i_am_on_the_page(world: &mut FrostTubeWorld, page_name: String) {
    assert_eq!(world.app.current_page, Page::Index);
}

#[then(expr = "the current page should be the {string} page")]
fn the_current_page_should_be_the_n_page(world: &mut FrostTubeWorld, page_name: String) {
    assert_eq!(world.app.current_page, Page::Index);
}

#[then(expr = "I should see a text field with a label of {string}")]
fn i_should_see_text_field(world: &mut FrostTubeWorld, label: String) {
    let mut ui = simulator(world.app.view());
    assert!(
        matches!(ui.find(label.as_str()), Ok(Text::Input { .. })),
        "Expected to find a text input with label '{label}'"
    );
}

#[when(expr = "I click the button {string}")]
fn i_click_the_button(world: &mut FrostTubeWorld, text: String) {
    let mut ui = simulator(world.app.view());
    helpers::find_button_by_text(&mut ui, &text)
        .unwrap_or_else(|| panic!("Expected to find a button with text '{text}'"));
    ui.click(text.as_str()).expect("Failed to click button text");
    for message in ui.into_messages() {
        world.app.update(message);
    }
}

#[then(expr = "I should be taken to the {string} page")]
fn i_should_be_taken_to_page(world: &mut FrostTubeWorld, page_name: String) {
    let expected = match page_name.as_str() {
        "Search Results" => Page::SearchResults,
        "Index" => Page::Index,
        _ => panic!("Unknown page: {page_name}"),
    };
    assert_eq!(world.app.current_page, expected);
}
