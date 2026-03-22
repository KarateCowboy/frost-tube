use cucumber::World;
use frost_tube::*;
mod steps;

#[derive(Debug, Default, World)]
pub struct FrostTubeWorld {
    app: App,
}

#[tokio::main]
async fn main() {
    FrostTubeWorld::run("tests/features").await;
}
