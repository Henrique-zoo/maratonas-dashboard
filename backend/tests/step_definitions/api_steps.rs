use cucumber::{gherkin::Step, then};
use serde_json::Value;

use crate::ApiWorld;

use super::single_column_table;

fn response_body(world: &ApiWorld) -> &str {
    &world
        .last_response
        .as_ref()
        .expect("a response should have been captured before asserting its body")
        .body
}

fn response_json(world: &ApiWorld) -> Value {
    let body = response_body(world);

    serde_json::from_str(body).unwrap_or_else(|error| {
        panic!("response body should be valid JSON: {error}\nresponse body: {body}")
    })
}

#[then(expr = "the response should be a JSON error with status {int}")]
fn response_should_be_json_error(world: &mut ApiWorld, expected_status: u16) {
    let response = world
        .last_response
        .as_ref()
        .expect("a response should have been captured");

    assert_eq!(response.status.as_u16(), expected_status);

    let payload = response_json(world);
    let object = payload
        .as_object()
        .unwrap_or_else(|| panic!("error response should be a JSON object, but was {payload}"));

    assert_eq!(
        object.len(),
        1,
        "error response should contain only the public error field: {payload}"
    );
    assert!(
        object.get("error").and_then(Value::as_str).is_some(),
        "error response should contain a string field named 'error': {payload}"
    );
}

fn normalize_pointer(pointer: &str) -> &str {
    match pointer {
        "$" => "",
        pointer if pointer.starts_with("$/") => &pointer[1..],
        pointer => pointer,
    }
}

fn value_at_pointer<'a>(payload: &'a Value, pointer: &str) -> &'a Value {
    let normalized = normalize_pointer(pointer);

    assert!(
        normalized.is_empty() || normalized.starts_with('/'),
        "invalid JSON Pointer {pointer:?}: use \"$\" for the root or a path beginning with '/'; response JSON: {payload}"
    );

    payload.pointer(normalized).unwrap_or_else(|| {
        panic!("JSON Pointer {pointer:?} was not found in response JSON: {payload}")
    })
}

fn array_at_pointer<'a>(payload: &'a Value, pointer: &str) -> &'a [Value] {
    let value = value_at_pointer(payload, pointer);

    value.as_array().unwrap_or_else(|| {
        panic!(
            "value at JSON Pointer {pointer:?} should be an array, but was {value}; response JSON: {payload}"
        )
    })
}

fn projected_value(value: &Value, field: &str, index: usize, payload: &Value) -> String {
    let projected = value.get(field).unwrap_or_else(|| {
        panic!("array item {index} has no field {field:?}; item: {value}; response JSON: {payload}")
    });

    match projected {
        Value::String(value) => value.clone(),
        Value::Number(_) | Value::Bool(_) | Value::Null => projected.to_string(),
        Value::Array(_) | Value::Object(_) => panic!(
            "field {field:?} in array item {index} should be a scalar value, but was {projected}; response JSON: {payload}"
        ),
    }
}

#[then("the response JSON should equal:")]
fn response_json_should_equal(world: &mut ApiWorld, #[step] step: &Step) {
    let expected_source = step
        .docstring()
        .expect("step should provide the expected JSON in a DocString");
    let expected: Value = serde_json::from_str(expected_source).unwrap_or_else(|error| {
        panic!("expected DocString should be valid JSON: {error}\nDocString: {expected_source}")
    });
    let actual = response_json(world);

    assert_eq!(
        actual, expected,
        "response JSON did not match the expected JSON"
    );
}

#[then(regex = r#"^the JSON value at "([^"]+)" should be (-?\d+)$"#)]
fn response_json_integer_at_pointer(world: &mut ApiWorld, pointer: String, expected: i64) {
    let payload = response_json(world);
    let actual = value_at_pointer(&payload, &pointer);

    assert_eq!(
        actual.as_i64(),
        Some(expected),
        "value at JSON Pointer {pointer:?} should be integer {expected}, but was {actual}; response JSON: {payload}"
    );
}

#[then(regex = r#"^the JSON value at "([^"]+)" should be "([^"]*)"$"#)]
fn response_json_string_at_pointer(world: &mut ApiWorld, pointer: String, expected: String) {
    let payload = response_json(world);
    let actual = value_at_pointer(&payload, &pointer);

    assert_eq!(
        actual.as_str(),
        Some(expected.as_str()),
        "value at JSON Pointer {pointer:?} should be string {expected:?}, but was {actual}; response JSON: {payload}"
    );
}

#[then(regex = r#"^the JSON array at "([^"]+)" should have length (\d+)$"#)]
fn response_json_array_should_have_length(world: &mut ApiWorld, pointer: String, expected: usize) {
    let payload = response_json(world);
    let actual = array_at_pointer(&payload, &pointer);

    assert_eq!(
        actual.len(),
        expected,
        "array at JSON Pointer {pointer:?} should have {expected} items, but had {}; response JSON: {payload}",
        actual.len()
    );
}

#[then(regex = r#"^the values of field "([^"]+)" in the JSON array at "([^"]+)" should be:$"#)]
fn response_json_array_field_values_should_equal(
    world: &mut ApiWorld,
    field: String,
    pointer: String,
    #[step] step: &Step,
) {
    let payload = response_json(world);
    let actual = array_at_pointer(&payload, &pointer)
        .iter()
        .enumerate()
        .map(|(index, item)| projected_value(item, &field, index, &payload))
        .collect::<Vec<_>>();
    let expected = single_column_table(step);

    assert_eq!(
        actual, expected,
        "values projected from field {field:?} at JSON Pointer {pointer:?} did not match the expected table; response JSON: {payload}"
    );
}

#[then(regex = r#"^the response body should contain "([^"]*)"$"#)]
fn response_body_should_contain(world: &mut ApiWorld, expected: String) {
    let body = response_body(world);

    assert!(
        body.contains(&expected),
        "response body should contain {expected:?}, but it did not; response body: {body}"
    );
}

#[then("the response JSON should be an empty array")]
fn response_json_should_be_empty_array(world: &mut ApiWorld) {
    let payload = response_json(world);
    let array = payload
        .as_array()
        .unwrap_or_else(|| panic!("response JSON should be an array, but was {payload}"));

    assert!(
        array.is_empty(),
        "response JSON should be an empty array, but had {} items: {payload}",
        array.len()
    );
}
