use crate::schema_bundle;
use proc_macro2::TokenStream;
use proc_quote::*;

pub enum FieldTypeDefinition<'a> {
    Singular(ValueTypeReference<'a>),
    Optional(ValueTypeReference<'a>),
    List(ValueTypeReference<'a>),
    Map {
        key: ValueTypeReference<'a>,
        value: ValueTypeReference<'a>,
    },
}

impl<'a> ToTokens for FieldTypeDefinition<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldTypeDefinition::Singular(ty) => {
                ty.to_tokens(tokens);
            }

            FieldTypeDefinition::Optional(ty) => {
                tokens.append_all(quote! {
                    Option<#ty>
                });
            }

            FieldTypeDefinition::List(ty) => {
                tokens.append_all(quote! {
                    Vec<#ty>
                });
            }

            FieldTypeDefinition::Map { key, value } => {
                tokens.append_all(quote! {
                    std::collections::BTreeMap<#key, #value>
                });
            }
        }
    }
}

pub enum ValueTypeReference<'a> {
    Primitive(schema_bundle::PrimitiveType),
    Enum(EnumReference<'a>),
    Type(TypeReference<'a>),
}

impl<'a> ToTokens for ValueTypeReference<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ValueTypeReference::Primitive(primitive) => primitive.to_tokens(tokens),
            ValueTypeReference::Enum(enum_ref) => enum_ref.to_tokens(tokens),
            ValueTypeReference::Type(type_ref) => type_ref.to_tokens(tokens),
        }
    }
}

pub struct TypeReference<'a> {
    pub(crate) bundle: &'a schema_bundle::SchemaBundleV1,
    pub(crate) qualified_name: &'a str,
}

impl<'a> ToTokens for TypeReference<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Look up the declaration for the type being referenced.
        let definition = self
            .bundle
            .type_definitions
            .iter()
            .find(|def| def.identifier.qualified_name == self.qualified_name)
            .expect("Type reference couldn't be resolved");

        tokens.append_all(definition.identifier.reference_path());
    }
}

pub struct EnumReference<'a> {
    pub(crate) bundle: &'a schema_bundle::SchemaBundleV1,
    pub(crate) qualified_name: &'a str,
}

impl<'a> ToTokens for EnumReference<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Look up the declaration for the type being referenced.
        let definition = self
            .bundle
            .enum_definitions
            .iter()
            .find(|def| def.identifier.qualified_name == self.qualified_name)
            .expect("Enum reference couldn't be resolved");

        tokens.append_all(definition.identifier.reference_path());
    }
}
