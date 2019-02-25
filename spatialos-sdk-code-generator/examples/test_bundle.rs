use std::str;

static TEST_BUNDLE: &[u8] = include_bytes!("../data/bundle.json");

fn main() {
    let contents = str::from_utf8(TEST_BUNDLE).unwrap();
    let bundle =
        spatialos_sdk_code_generator::schema_bundle::load_bundle(contents).expect("Failed to parse bundle contents");
    let example_generated = spatialos_sdk_code_generator::generate(&bundle, "example", &["crate::improbable", "crate::example"]).expect("Code generation failed");
    let improbable_generated = spatialos_sdk_code_generator::generate(&bundle, "improbable", &["crate::improbable"]).expect("Code generation failed");
    println!("// Generated from the improbable package.");
    println!("{}", improbable_generated);
    println!("// Generated from the example package.");
    println!("{}", example_generated);
}
