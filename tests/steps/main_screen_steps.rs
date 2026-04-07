use crate::FrostTubeWorld;
use crate::helpers;

use cucumber::{given, then, when};
use frost_tube::services::Video;
use frost_tube::*;
use rectum::InnerTubeClient;

use iced_test::selector::Text;
use iced_test::simulator;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[given("I have a new application instance")]
async fn i_have_a_new_application_instance(world: &mut FrostTubeWorld) {
    let server = MockServer::start().await;
    let mut app = App::default();
    app.client = InnerTubeClient::builder().base_url(server.uri()).build();
    world.mock_server = Some(server);
    world.app = app;
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
    ui.click(text.as_str())
        .expect("Failed to click button text");
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }
}

#[when(expr = "I search {string}")]
async fn i_search(world: &mut FrostTubeWorld, query: String) {
    let server = world
        .mock_server
        .as_ref()
        .expect("MockServer not initialized");

    let response_body = innertube_search_fixture();

    Mock::given(method("POST"))
        .and(path("/youtubei/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(server)
        .await;

    let mut ui = simulator(world.app.view());
    ui.click("Search").expect("Failed to click search input");
    ui.typewrite(&query);
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }

    let mut ui = simulator(world.app.view());
    ui.click("Go").expect("Failed to click Go button");
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }

    let result = world
        .app
        .client
        .search(&query)
        .await
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
        .map_err(|e| e.to_string());
    let _ = world.app.update(Message::SearchResultsReceived(result));
}

#[then("the I should see the search results entries")]
fn i_should_see_the_search_results_entries(world: &mut FrostTubeWorld) {
    assert!(
        !world.app.search_results.is_empty(),
        "Expected search results to be populated"
    );
    assert_eq!(world.app.search_results.len(), 2);

    let mut ui = simulator(world.app.view());
    for video in &world.app.search_results {
        assert!(
            ui.find(video.title.as_str()).is_ok(),
            "Expected to find search result: {}",
            video.title
        );
    }
}

#[when("I search and the API returns an error")]
async fn i_search_and_api_returns_error(world: &mut FrostTubeWorld) {
    let server = world
        .mock_server
        .as_ref()
        .expect("MockServer not initialized");

    Mock::given(method("POST"))
        .and(path("/youtubei/v1/search"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(server)
        .await;

    let mut ui = simulator(world.app.view());
    ui.click("Search").expect("Failed to click search input");
    ui.typewrite("anything");
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }

    let mut ui = simulator(world.app.view());
    ui.click("Go").expect("Failed to click Go button");
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }

    let result = world
        .app
        .client
        .search("anything")
        .await
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
        .map_err(|e| e.to_string());
    let _ = world.app.update(Message::SearchResultsReceived(result));
}

#[then("I should see an error message on screen")]
fn i_should_see_error_message(world: &mut FrostTubeWorld) {
    let err = world
        .app
        .error_message
        .as_ref()
        .expect("Expected error_message to be set");
    let expected_text = format!("Error: {err}");
    let mut ui = simulator(world.app.view());
    assert!(
        ui.find(expected_text.as_str()).is_ok(),
        "Expected to see '{expected_text}' on screen"
    );
}

#[then("I should see an error message modal")]
fn i_should_see_an_error_message_modal(world: &mut FrostTubeWorld) {
    let err_message = world
        .app
        .error_message
        .as_ref()
        .expect("Expected an error message to be set, but it was not");
    let mut ui = simulator(world.app.view());
    assert!(
        ui.find(err_message.as_str()).is_ok(),
        "Expected alert to display error: {err_message}"
    );
    assert!(
        matches!(ui.find("Dismiss"), Ok(Text::Raw { .. })),
        "Expected modal to have a 'Dismiss' button"
    );
}

#[when(expr = "I click the video {string}")]
fn i_click_the_video(world: &mut FrostTubeWorld, title: String) {
    let mut ui = simulator(world.app.view());
    ui.click(title.as_str())
        .unwrap_or_else(|_| panic!("Expected to find clickable video '{title}'"));
    for message in ui.into_messages() {
        let _ = world.app.update(message);
    }
}

#[then(expr = "I should be taken to the {string} page")]
fn i_should_be_taken_to_page(world: &mut FrostTubeWorld, page_name: String) {
    let expected = match page_name.as_str() {
        "Search Results" => Page::SearchResults,
        "Index" => Page::Index,
        "Video Detail" => Page::VideoDetail { video_id: String::new() },
        _ => panic!("Unknown page: {page_name}"),
    };
    assert!(
        std::mem::discriminant(&world.app.current_page) == std::mem::discriminant(&expected),
        "Expected page {:?}, got {:?}",
        expected,
        world.app.current_page
    );
}

#[then(expr = "I should see the video title {string}")]
fn i_should_see_the_video_title(world: &mut FrostTubeWorld, title: String) {
    let mut ui = simulator(world.app.view());
    assert!(
        ui.find(title.as_str()).is_ok(),
        "Expected to see video title '{title}' on the detail page"
    );
}

fn innertube_search_fixture() -> serde_json::Value {
    serde_json::json!({
        "estimatedResults": "2",
        "contents": {
            "twoColumnSearchResultsRenderer": {
                "primaryContents": {
                    "sectionListRenderer": {
                        "contents": [{
                            "itemSectionRenderer": {
                                "contents": [
                                    {
                                        "videoRenderer": {
                                            "videoId": "abc123",
                                            "title": { "runs": [{ "text": "Kaze Fuiteru - Official Music Video" }] },
                                            "lengthText": { "simpleText": "4:49" },
                                            "viewCountText": { "simpleText": "1,000 views" },
                                            "ownerText": { "runs": [{ "text": "TestChannel" }] }
                                        }
                                    },
                                    {
                                        "videoRenderer": {
                                            "videoId": "def456",
                                            "title": { "runs": [{ "text": "Kaze Fuiteru Live at Budokan" }] },
                                            "lengthText": { "simpleText": "6:12" },
                                            "viewCountText": { "simpleText": "500 views" },
                                            "ownerText": { "runs": [{ "text": "TestChannel2" }] }
                                        }
                                    }
                                ]
                            }
                        }]
                    }
                }
            }
        }
    })
}
