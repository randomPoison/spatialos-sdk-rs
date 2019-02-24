use crate::schema_bundle::*;
use quote::*;
use std::collections::BTreeMap;

pub mod schema_bundle;

// TODO: Create a proper error type to return.
pub fn generate(
    bundle: &SchemaBundle,
    package: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let bundle = bundle.v1.as_ref().ok_or("Only v1 bundle is supported")?;
    let null_span = proc_macro2::Span::call_site();

    // Create the module that is the root of the generated code.
    let mut module = Module::default();

    bundle
        .component_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .for_each(|component_def| {
            let ident = syn::Ident::new(&component_def.identifier.name, null_span);

            let generated = match &component_def.data_definition {
                ComponentDataDefinition::Inline(fields) => {
                    let fields = fields.iter().map(|field_def| {
                        let ident = syn::Ident::new(&field_def.identifier.name, null_span);
                        let ty = &field_def.ty;
                        quote! {
                            #ident: #ty
                        }
                    });

                    quote! {
                        struct #ident {
                            #( #fields ),*
                        }
                    }
                }

                ComponentDataDefinition::TypeReference(type_reference) => {
                    quote! {
                        struct #ident(#type_reference);
                    }
                }
            };

            let module_path = component_def.identifier.module_path();
            let module = module.get_submodule(module_path);
            module.items.push(generated);
        });

    bundle
        .type_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .for_each(|type_def| {
            let ident = syn::Ident::new(&type_def.identifier.name, null_span);

            let fields = type_def.field_definitions.iter().map(|field_def| {
                let ident = syn::Ident::new(&field_def.identifier.name, null_span);
                let ty = &field_def.ty;
                quote! {
                    pub #ident: #ty
                }
            });

            let generated = quote! {
                struct #ident {
                    #( #fields ),*
                }
            };

            let module_path = type_def.identifier.module_path();
            let module = module.get_submodule(module_path);
            module.items.push(generated);
        });

    bundle
        .enum_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .for_each(|enum_def| {
            let ident = syn::Ident::new(&enum_def.identifier.name, null_span);
            let values = &enum_def.value_definitions;

            let generated = quote! {
                enum #ident {
                    #( #values ),*
                }
            };

            let module_path = enum_def.identifier.module_path();
            let module = module.get_submodule(module_path);
            module.items.push(generated);
        });

    Ok(module.into_token_stream().to_string())
}

#[derive(Debug, Clone, Default)]
struct Module {
    // NOTE: We track the modules in a `BTreeMap` because it provides deterministic
    // iterator ordering. Deterministic ordering in the generated code is useful
    // when diffing changes and debugging the generated code.
    modules: BTreeMap<String, Module>,
    items: Vec<proc_macro2::TokenStream>,
}

impl Module {
    fn get_submodule<I, S>(&mut self, path: I) -> &mut Module
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut module = self;
        for ident in path {
            module = module.modules.entry(ident.into()).or_default();
        }
        module
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for (ident, module) in &self.modules {
            let ident = syn::Ident::new(ident, proc_macro2::Span::call_site());
            tokens.append_all(quote! {
                pub mod #ident {
                    #module
                }
            });
        }

        for item in &self.items {
            tokens.append_all(quote! {
                pub #item
            });
        }
    }
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
