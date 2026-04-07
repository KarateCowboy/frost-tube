use frost_tube::invidious::InvidiousClient;
use frost_tube::services::VideoService;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn search_returns_error_on_non_json_response() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&server)
        .await;

    let client = InvidiousClient::new(&server.uri());
    let result = client.search("test").await;

    let err = result.unwrap_err();
    assert!(err.contains("500"), "Expected error to mention status code 500, got: {err}");
}

#[tokio::test]
async fn search_returns_videos_on_valid_response() {
    let server = MockServer::start().await;

    let body = serde_json::json!([
        { "type": "video", "title": "Test Video", "videoId": "abc123" }
    ]);

    Mock::given(method("GET"))
        .and(path("/api/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&body))
        .mount(&server)
        .await;

    let client = InvidiousClient::new(&server.uri());
    let result = client.search("test").await;

    assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
    let videos = result.unwrap();
    assert_eq!(videos.len(), 1);
    assert_eq!(videos[0].title, "Test Video");
}
