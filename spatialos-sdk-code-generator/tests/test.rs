use maplit::hashmap;
use spatialos_sdk_code_generator::*;
use std::str;

static TEST_BUNDLE: &[u8] = include_bytes!("../data/bundle.json");

#[test]
fn deserialize_bundle() {
    let contents = str::from_utf8(TEST_BUNDLE).unwrap();
    let bundle = schema_bundle::load_bundle(contents).expect("Failed to parse bundle contents");
    let _ = generate(
        &bundle,
        "example",
        &hashmap! {
            "improbable" => "spatialos_sdk::schema",
            "example" => "crate::schema",
        },
    )
    .expect("Code generation failed");
}
