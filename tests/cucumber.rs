use cucumber::World;
use frost_tube::*;
mod steps;
mod helpers;

#[derive(Debug, Default, World)]
pub struct FrostTubeWorld {
    app: App,
    mock_service: helpers::MockVideoService,
}

#[tokio::main]
async fn main() {
    FrostTubeWorld::run("tests/features").await;
}
