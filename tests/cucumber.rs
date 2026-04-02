use cucumber::World;
use frost_tube::*;
mod steps;
mod helpers;

#[derive(Debug, World)]
pub struct FrostTubeWorld {
    app: App,
    mock_server: Option<wiremock::MockServer>,
}

impl Default for FrostTubeWorld {
    fn default() -> Self {
        Self {
            app: App::default(),
            mock_server: None,
        }
    }
}

#[tokio::main]
async fn main() {
    FrostTubeWorld::run("tests/features").await;
}
