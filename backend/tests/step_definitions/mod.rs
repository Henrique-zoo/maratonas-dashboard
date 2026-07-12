use cucumber::gherkin::Step;

pub(crate) mod competition_steps;
pub(crate) mod institution_steps;
pub(crate) mod organization_steps;
pub(crate) mod team_steps;

pub(crate) fn single_column_table(step: &Step) -> Vec<String> {
    let table = step
        .table()
        .expect("step should provide a single-column data table");

    table
        .rows
        .iter()
        .map(|row| {
            assert_eq!(row.len(), 1, "expected a single-column data table row");
            row[0].clone()
        })
        .collect()
}

pub(crate) fn assert_table_values(step: &Step, actual: Vec<String>) {
    let expected = single_column_table(step);

    assert_eq!(actual, expected);
}
