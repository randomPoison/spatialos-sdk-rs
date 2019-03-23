use maplit::hashmap;

fn main() {
    cargo_spatial::codegen::build(
        "example",
        &hashmap! {
            "improbable" => "spatialos_sdk::schema",
            "example" => "crate::generated_code",
            "example.subpackage" => "crate::generated_code",
        },
    )
    .expect("Code generation failed");
}
