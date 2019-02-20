fn main() {
    // Use cargo-spatial to generate the schema file from all the bundles.
    let spatial_lib_dir = env::var("SPATIAL_LIB_DIR").expect("SPATIAL_LIB_DIR environment variable not set");
    let schema_dir = Path::new(&spatial_lib_dir).join("std-lib");
    let bundle_path = cargo_spatial::generate_bundle(schema_dir).expect("Failed to generate bundle");

    // Use the code generator to generate the Rust types from the schema bundle.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated.rs");
    spatialos_sdk_code_generator::generate(bundle_path, dest_path);
}
