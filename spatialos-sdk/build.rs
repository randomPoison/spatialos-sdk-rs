use std::{
    env,
    fs::{self, File},
    path::Path,
};

fn main() {
    // Use the code generator to generate the Rust types from the schema bundle.
    cargo_spatial::codegen::build(
        "improbable",
        &["crate::schema::improbable"],
    )
    .expect("Failed to generate code from bundle");
}
