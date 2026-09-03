use cucumber::{gherkin::Step, given, then, when};

use crate::ApiWorld;

use super::assert_table_values;

#[given(regex = r#"^an institution named "([^"]+)" exists$"#)]
fn institution_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.institution_exists(&name),
        "expected institution {name} to exist in the home filter context",
    );
}

#[when(regex = r#"^I select "([^"]+)" in the institution filter$"#)]
fn select_institution(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_institution(&name);
}

#[then("the team filter options should be:")]
fn team_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.team_option_names());
}
