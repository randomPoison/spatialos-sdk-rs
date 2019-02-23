use crate::schema_bundle::SchemaBundle;
use quote::quote;

pub mod schema_bundle;

// TODO: Create a proper error type to return.
pub fn generate(
    bundle: &SchemaBundle,
    package: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let bundle = bundle.v1.as_ref().ok_or("Only v1 bundle is supported")?;
    let null_span = proc_macro2::Span::call_site();

    // TODO: Filter to only the components that are part of the crate that's currently
    // being processed.
    let components = bundle
        .component_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .map(|component_def| {
            // println!("Component identifier: {:?}", component_def.identifier);

            let ident = syn::Ident::new(&component_def.identifier.name, null_span);

            let fields = component_def.field_definitions.iter().map(|field_def| {
                let ident = syn::Ident::new(&field_def.identifier.name, null_span);
                let ty = &field_def.ty;
                quote! {
                    pub #ident: #ty
                }
            });

            quote! {
                pub struct #ident {
                    #( #fields ),*
                }
            }
        });

    let result = quote! {
        #( #components )*
    }
    .to_string();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use std::str;

    static TEST_BUNDLE: &[u8] = include_bytes!("../data/bundle.json");

    #[test]
    fn deserialize_bundle() {
        let contents = str::from_utf8(TEST_BUNDLE).unwrap();
        let bundle =
            crate::schema_bundle::load_bundle(contents).expect("Failed to parse bundle contents");
        let _ = crate::generate(&bundle, "example").expect("Code generation failed");
    }
}
