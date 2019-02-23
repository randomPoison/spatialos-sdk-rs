use crate::schema_bundle::SchemaBundle;
use std::path::{Path, PathBuf};

pub mod generator;
pub mod schema_bundle;

// TODO: Does it actually make sense to have this function do file I/O? Or should it
// be given the already-loaded files? Or would there be a better way to abstract
// over the two?
//
// TODO: Is there a better way to specify the list of input files?
//
// TODO: Create a proper error type to return.
pub fn generate<P>(bundle: SchemaBundle) -> Result<Schema, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::str;

    static TEST_BUNDLE: &[u8] = include_bytes!("../data/standard_library.json");

    #[test]
    fn deserialize_bundle() {
        let contents = str::from_utf8(TEST_BUNDLE).unwrap();

        crate::generate(vec![contents.into()], "out/test_generated.rs")
            .expect("Code generation failed");
    }
}
