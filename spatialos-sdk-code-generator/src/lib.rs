use crate::schema_bundle::*;
use quote::*;
use std::collections::BTreeMap;

pub mod schema_bundle;

// TODO: Create a proper error type to return.
pub fn generate(
    bundle: &SchemaBundle,
    package: &str,
    prelude: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let bundle = bundle.v1.as_ref().ok_or("Only v1 bundle is supported")?;
    let null_span = proc_macro2::Span::call_site();

    // Generate the prelude that needs to be injected into all generated modules.
    let prelude = prelude.iter().map(|path| {
        syn::parse_str::<proc_macro2::TokenStream>(path).expect("Invalid path in prelude")
    });
    let prelude = quote! {
        #( use #prelude; )*
    };

    // Create the module that is the root of the generated code.
    let mut module = Module::new(prelude, true);

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
                        let ty = field_def.ty.quotable(&bundle);
                        quote! {
                            #ident: #ty
                        }
                    });

                    quote! {
                        struct #ident {
                            #( pub #fields ),*
                        }
                    }
                }

                ComponentDataDefinition::TypeReference(type_reference) => {
                    let ty_ref = type_reference.quotable(&bundle);
                    quote! {
                        struct #ident(#ty_ref);
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
                let ty = field_def.ty.quotable(&bundle);
                quote! {
                    #ident: #ty
                }
            });

            let generated = quote! {
                struct #ident {
                    #( pub #fields ),*
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

    let raw_generated = module.into_token_stream().to_string();
    let generated = rustfmt(raw_generated.clone()).unwrap_or(raw_generated);
    Ok(generated)
}

pub fn rustfmt<S>(module: S) -> Result<String, Box<dyn std::error::Error>>
where
    S: Into<String>,
{
    use std::{
        io::Write,
        process::{Command, Stdio},
        str,
    };

    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(module.into().as_bytes())?;

    let output = child.wait_with_output()?;
    let formatted = str::from_utf8(&output.stdout[..])?.into();

    Ok(formatted)
}

#[derive(Debug, Clone)]
struct Module {
    is_root: bool,
    prelude: proc_macro2::TokenStream,
    // NOTE: We track the modules in a `BTreeMap` because it provides deterministic
    // iterator ordering. Deterministic ordering in the generated code is useful
    // when diffing changes and debugging the generated code.
    modules: BTreeMap<String, Module>,
    items: Vec<proc_macro2::TokenStream>,
}

impl Module {
    fn new(prelude: proc_macro2::TokenStream, is_root: bool) -> Self {
        Module {
            is_root,
            prelude,
            modules: Default::default(),
            items: Default::default(),
        }
    }

    fn get_submodule<I, S>(&mut self, path: I) -> &mut Module
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let default_module = Module::new(self.prelude.clone(), false);
        let mut module = self;
        for ident in path {
            module = module
                .modules
                .entry(ident.into())
                .or_insert_with(|| default_module.clone());
        }
        module
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.is_root {
            tokens.append_all(self.prelude.clone());
        }

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
