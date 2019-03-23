use maplit::hashmap;

fn main() {
    cargo_spatial::codegen::build(
        "example",
        &hashmap! {
            "improbable" => "spatialos_sdk::schema::improbable",
            "example" => "crate::generated_code::example",
        },
    )
    .expect("Code generation failed");
}
