use maplit::hashmap;

fn main() {
    // Use the code generator to generate the Rust types from the schema bundle.
    cargo_spatial::codegen::build(
        "improbable",
        &hashmap! { "improbable" => "crate::schema" },
    )
    .expect("Failed to generate code from bundle");
}
