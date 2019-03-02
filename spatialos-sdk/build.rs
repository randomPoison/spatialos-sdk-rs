use std::{
    env,
    fs::{self, File},
    path::Path,
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let bundle_path = Path::new(&out_dir).join("bundle.json");

    // Use cargo-spatial to generate the schema file from all the bundles.
    let spatial_lib_dir =
        env::var("SPATIAL_LIB_DIR").expect("SPATIAL_LIB_DIR environment variable not set");
    cargo_spatial::codegen::compile_schemas(&spatial_lib_dir, Some(&bundle_path), None, &[])
        .expect("Failed to compile schema files");

    // Load the source bundle.
    let bundle_file = File::open(&bundle_path).expect("Failed to open bundle.json");
    let bundle = serde_json::from_reader(bundle_file).expect("Failed to deserialize bundle.json");

    // Use the code generator to generate the Rust types from the schema bundle.
    let generated = spatialos_sdk_code_generator::generate(
        &bundle,
        "improbable",
        &["crate::schema::improbable"],
    )
    .expect("Failed to generate code from bundle");

    let dest_path = Path::new(&out_dir).join("generated.rs");
    fs::write(dest_path, generated).expect("Failed to write generated code to file");
}
