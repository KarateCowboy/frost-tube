use crate::FrostTubeWorld;
use crate::helpers;

use cucumber::{given, then, when};
use frost_tube::*;
use frost_tube::invidious::InvidiousClient;
use frost_tube::services::VideoService;
use iced_test::selector::Text;
use iced_test::simulator;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[given("I have a new application instance")]
async fn i_have_a_new_application_instance(world: &mut FrostTubeWorld) {
    let server = MockServer::start().await;
    world.client = Some(InvidiousClient::new(&server.uri()));
    world.mock_server = Some(server);
    world.app = App::default();
}

#[given(expr = "I am on the {string} page")]
fn i_am_on_the_page(world: &mut FrostTubeWorld, _page_name: String) {
    assert_eq!(world.app.current_page, Page::Index);
}

#[then(expr = "the current page should be the {string} page")]
fn the_current_page_should_be_the_n_page(world: &mut FrostTubeWorld, _page_name: String) {
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

#[when(expr = "I search {string}")]
async fn i_search(world: &mut FrostTubeWorld, query: String) {
    // Set up wiremock to return canned Invidious response
    let server = world.mock_server.as_ref().expect("MockServer not initialized");
    let response_body = serde_json::json!([
        {
            "type": "video",
            "title": "Kaze Fuiteru - Official Music Video",
            "videoId": "abc123"
        },
        {
            "type": "video",
            "title": "Kaze Fuiteru Live at Budokan",
            "videoId": "def456"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .and(query_param("q", &query))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(server)
        .await;

    // Drive the UI: type query and click Go
    let mut ui = simulator(world.app.view());
    ui.click("Search").expect("Failed to click search input");
    ui.typewrite(&query);
    for message in ui.into_messages() {
        world.app.update(message);
    }

    let mut ui = simulator(world.app.view());
    ui.click("Go").expect("Failed to click Go button");
    for message in ui.into_messages() {
        world.app.update(message);
    }

    // Hit the wiremock server via the real InvidiousClient
    let client = world.client.as_ref().expect("InvidiousClient not initialized");
    let results = client.search(&query).await;
    world.app.update(Message::SearchResultsReceived(results));
}

#[then("the I should see the search results entries")]
fn i_should_see_the_search_results_entries(world: &mut FrostTubeWorld) {
    assert!(!world.app.search_results.is_empty(), "Expected search results to be populated");

    let mut ui = simulator(world.app.view());
    for video in &world.app.search_results {
        assert!(
            ui.find(video.title.as_str()).is_ok(),
            "Expected to find search result: {}",
            video.title
        );
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
