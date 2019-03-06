fn main() {
    cargo_spatial::codegen::build(
        "example",
        &["spatialos_sdk::schema", "crate::generated_code"],
    )
    .expect("Code generation failed");
}
