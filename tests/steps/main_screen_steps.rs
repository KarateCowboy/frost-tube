use crate::FrostTubeWorld;

use cucumber::{given, then};
use frost_tube::*;

#[given("I have a new application instance")]
fn i_have_a_new_application_instance(world: &mut FrostTubeWorld) {
    world.app = App::default();
}
#[then(expr = "the current page should be the {string} page")]
fn the_current_page_should_be_the_n_page(world: &mut FrostTubeWorld, page_name: String) {
    assert_eq!(world.app.current_page, Page::Index);
}
