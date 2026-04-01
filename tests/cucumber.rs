use cucumber::World;
use frost_tube::*;
use frost_tube::invidious::InvidiousClient;
mod steps;
mod helpers;

#[derive(Debug, World)]
pub struct FrostTubeWorld {
    app: App,
    mock_server: Option<wiremock::MockServer>,
    client: Option<InvidiousClient>,
}

impl Default for FrostTubeWorld {
    fn default() -> Self {
        Self {
            app: App::default(),
            mock_server: None,
            client: None,
        }
    }
}

#[tokio::main]
async fn main() {
    FrostTubeWorld::run("tests/features").await;
}
