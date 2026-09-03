use cucumber::{given, when};

use crate::ApiWorld;

#[given(regex = r#"^a team named "([^"]+)" exists$"#)]
fn team_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.team_exists(&name),
        "expected team {name} to exist in the home filter context",
    );
}

#[when(regex = r#"^I select "([^"]+)" in the team filter$"#)]
fn select_team(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_team(&name);
}
