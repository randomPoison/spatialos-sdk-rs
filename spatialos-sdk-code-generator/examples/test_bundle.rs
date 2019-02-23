
    use std::str;

    static TEST_BUNDLE: &[u8] = include_bytes!("../data/bundle.json");

    fn main() {
        let contents = str::from_utf8(TEST_BUNDLE).unwrap();
        let bundle =
            spatialos_sdk_code_generator::schema_bundle::load_bundle(contents).expect("Failed to parse bundle contents");
        let generated = spatialos_sdk_code_generator::generate(bundle).expect("Code generation failed");
        println!("{}", generated);
    }
