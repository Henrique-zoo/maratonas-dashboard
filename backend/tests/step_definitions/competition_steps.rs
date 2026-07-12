use cucumber::{gherkin::Step, given, then, when};

use crate::ApiWorld;

use super::assert_table_values;

#[given(regex = r#"^existe a competicao "([^"]+)"$"#)]
fn competition_exists(world: &mut ApiWorld, name: String) {
    assert!(
        world.home.competition_exists(&name),
        "expected competition {name} to exist in the home filter context",
    );
}

#[when(regex = r#"^seleciono "([^"]+)" no filtro de competicao$"#)]
fn select_competition(world: &mut ApiWorld, name: String) {
    world.home.request_count_before_last_interaction = world.request_count;
    world.home.select_competition(&name);
}

#[then("as opcoes do filtro de instituicoes devem ser:")]
fn institution_filter_options(world: &mut ApiWorld, #[step] step: &Step) {
    assert_table_values(step, world.home.institution_option_names());
}
