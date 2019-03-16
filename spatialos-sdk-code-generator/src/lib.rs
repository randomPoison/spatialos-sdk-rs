use crate::schema_bundle::*;
use heck::SnekCase;
use proc_macro2::TokenStream;
use proc_quote::*;
use std::collections::BTreeMap;
use syn::Ident;

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
    let default_module = Module::new(prelude);

    let spatialos_sdk = if package == "improbable" {
        quote! { crate }
    } else {
        quote! { spatialos_sdk }
    };

    // Define variables for commonly-referenced types so that we don't have to type
    // out the fully-qualified paths every time.
    let schema_component_data =
        quote! { #spatialos_sdk::worker::internal::schema::SchemaComponentData };
    let schema_component_update =
        quote! { #spatialos_sdk::worker::internal::schema::SchemaComponentUpdate };
    let schema_command_request =
        quote! { #spatialos_sdk::worker::internal::schema::SchemaCommandRequest };
    let schema_command_response =
        quote! { #spatialos_sdk::worker::internal::schema::SchemaCommandResponse };

    // Track all generated modules in a `BTreeMap` in order to get deterministic
    // iteration ordering, which is useful when doing diffs when testing.
    let mut modules = BTreeMap::new();

    bundle
        .component_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .for_each(|component_def| {
            let ident = Ident::new(&component_def.identifier.name, null_span);

            let submodule_name = component_def.identifier.name.to_snek_case();
            let submodule_ident = Ident::new(&submodule_name, null_span);

            let struct_definition = match &component_def.data_definition {
                ComponentDataDefinition::Inline(fields) => {
                    let fields = fields.iter().map(|field_def| {
                        let ident = Ident::new(&field_def.identifier.name, null_span);
                        let ty = field_def.ty.quotable(&bundle);
                        quote! {
                            #ident: #ty
                        }
                    });

                    quote! {
                        pub struct #ident {
                            #( pub #fields ),*
                        }
                    }
                }

                ComponentDataDefinition::TypeReference(type_reference) => {
                    let ty_ref = type_reference.quotable(&bundle);
                    quote! {
                        pub struct #ident(#ty_ref);
                    }
                }
            };

            let component_id = component_def.component_id;
            let impls = quote! {
                impl #spatialos_sdk::worker::component::Component for #ident {
                    type Update = #submodule_ident::Update;
                    type CommandRequest = #submodule_ident::CommandRequest;
                    type CommandResponse = #submodule_ident::CommandResponse;

                    const ID: #spatialos_sdk::worker::component::ComponentId = #component_id;

                    fn from_data(_data: &#schema_component_data) -> Result<Self, String> {
                        unimplemented!()
                    }

                    fn from_update(_update: &#schema_component_update) -> Result<Self::Update, String> {
                        unimplemented!()
                    }

                    fn from_request(_request: &#schema_command_request) -> Result<Self::CommandRequest, String> {
                        unimplemented!()
                    }

                    fn from_response(_response: &#schema_command_response) -> Result<Self::CommandResponse, String> {
                        unimplemented!()
                    }

                    fn to_data(_data: &Self) -> Result<#schema_component_data, String> {
                        unimplemented!()
                    }

                    fn to_update(_update: &Self::Update) -> Result<#schema_component_update, String> {
                        unimplemented!()
                    }

                    fn to_request(_request: &Self::CommandRequest) -> Result<#schema_command_request, String> {
                        unimplemented!()
                    }

                    fn to_response(_response: &Self::CommandResponse) -> Result<#schema_command_response, String> {
                        unimplemented!()
                    }

                    fn get_request_command_index(_request: &Self::CommandRequest) -> u32 {
                        unimplemented!()
                    }

                    fn get_response_command_index(_response: &Self::CommandResponse) -> u32 {
                        unimplemented!()
                    }
                }
            };

            let associated_types = quote! {
                pub struct Update;
                pub enum CommandRequest {}
                pub enum CommandResponse {}
            };

            let module_path = component_def.identifier.module_path();
            let module = get_submodule(&mut modules, module_path, &default_module);
            module.items.push(struct_definition);
            module.items.push(impls);

            let submodule = get_submodule(&mut module.modules, std::iter::once(submodule_name), &default_module);
            submodule.items.push(associated_types);
        });

    bundle
        .type_definitions
        .iter()
        .filter(|def| def.identifier.qualified_name.starts_with(package))
        .for_each(|type_def| {
            let ident = Ident::new(&type_def.identifier.name, null_span);

            let fields = type_def.field_definitions.iter().map(|field_def| {
                let ident = Ident::new(&field_def.identifier.name, null_span);
                let ty = field_def.ty.quotable(&bundle);
                quote! {
                    #ident: #ty
                }
            });

            let generated = quote! {
                pub struct #ident {
                    #( pub #fields ),*
                }
            };

            let module_path = type_def.identifier.module_path();
            let module = get_submodule(&mut modules, module_path, &default_module);
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
                pub enum #ident {
                    #( #values ),*
                }
            };

            let module_path = enum_def.identifier.module_path();
            let module = get_submodule(&mut modules, module_path, &default_module);
            module.items.push(generated);
        });

    // Generate the code for each of the modules.
    let module_names = modules.keys().map(|name| Ident::new(name, null_span));
    let modules = modules.values();
    let raw_generated = quote! {
        #(
            #[allow(unused_imports)]
            pub mod #module_names {
                #modules
            }
        )*
    }
    .to_string();

    // Attempt to use rustfmt to format the code in order to help with debugging.
    // If this fails for any reason, we simply default to using the unformatted
    // code. This ensures that code generation can still work even if the user
    // doesn't have rustfmt installed.
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
    if !output.status.success() {
        return Err("Failed to format the code I guess".into());
    }

    let formatted = str::from_utf8(&output.stdout[..])?.into();
    Ok(formatted)
}

#[derive(Debug, Clone)]
struct Module {
    /// The import prelude to be injected at the beginning of the module.
    prelude: TokenStream,

    /// The items included in the module.
    items: Vec<TokenStream>,

    // NOTE: We track the modules in a `BTreeMap` because it provides deterministic
    // iterator ordering. Deterministic ordering in the generated code is useful
    // when diffing changes and debugging the generated code.
    modules: BTreeMap<String, Module>,
}

impl Module {
    fn new(prelude: TokenStream) -> Self {
        Module {
            prelude,
            modules: Default::default(),
            items: Default::default(),
        }
    }
}

impl ToTokens for Module {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Inject the prelude at the beginning of the module.
        tokens.append_all(self.prelude.clone());

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
                #item
            });
        }
    }
}

fn get_submodule<'a, I, S>(
    modules: &'a mut BTreeMap<String, Module>,
    path: I,
    default_module: &Module,
) -> &'a mut Module
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    // Extract the first module based on the first segment of the path.
    let mut iterator = path.into_iter();
    let first = iterator.next().expect("`path` was empty");
    let mut module = modules
        .entry(first.into())
        .or_insert_with(|| default_module.clone());

    // Iterate over the remaining segments of the path, getting or creating all
    // intermediate modules.
    for ident in iterator {
        module = module
            .modules
            .entry(ident.into())
            .or_insert_with(|| default_module.clone());
    }

    module
}
