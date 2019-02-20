use std::path::Path;

pub mod generator;
pub mod schema_bundle;

pub fn generate<P, Q>(bundle_path: P, output_file: Q) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    // Steps:
    //
    // 1. Find all schema files in dependencies. (cargo-spatial)
    // 2. Run schema compiler to generate AST/bundle file. (cargo-spatial)
    // 3. Generate code for schema files defined in the current crate.
    unimplemented!("TODO: Implement code generation");
}

#[cfg(test)]
mod tests {
    use crate::generator;
    use crate::schema_bundle;
    use std::str;

    static TEST_BUNDLE: &[u8] = include_bytes!("../data/test.bundle.json");

    #[test]
    fn deserialize_bundle() {
        let contents = str::from_utf8(TEST_BUNDLE).unwrap();

        let bundle = schema_bundle::load_bundle(&contents);
        assert!(
            bundle.is_ok(),
            "Schema bundle contains an error: {:?}",
            bundle.err().unwrap()
        );
        println!("Bundle contents: {:#?}", bundle);
        println!(
            "Generated code: {}",
            generator::generate_code(bundle.unwrap())
        );
    }
}
