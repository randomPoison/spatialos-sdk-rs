fn main() {
    cargo_spatial::codegen::build(
        "example",
        &["spatialos_sdk::schema::improbable", "crate::generated_code::example"],
    )
    .expect("Code generation failed");
}
