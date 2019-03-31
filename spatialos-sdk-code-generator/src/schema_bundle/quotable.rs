use crate::{schema_bundle, Context};
use proc_macro2::{Ident, Span, TokenStream};
use proc_quote::*;

#[derive(Clone, Debug)]
pub struct FieldDefinition<'a> {
    pub identifier: &'a schema_bundle::Identifier,
    pub field_id: u32,
    pub transient: bool,
    pub ty: FieldTypeDefinition<'a>,
    pub annotations: &'a Vec<schema_bundle::Annotation>,
}

impl<'a> ToTokens for FieldDefinition<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new(&self.identifier.name, Span::call_site());
        let ty = &self.ty;
        tokens.append_all(quote! {
            #ident: #ty
        })
    }
}

#[derive(Clone, Debug)]
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
    fn to_tokens(&self, tokens: &mut TokenStream) {
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct TypeReference<'a> {
    pub(crate) context: Context<'a>,
    pub(crate) qualified_name: &'a str,
}

impl<'a> ToTokens for TypeReference<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Look up the declaration for the type being referenced.
        let definition = self
            .context
            .bundle
            .type_definitions
            .iter()
            .find(|def| def.identifier.qualified_name == self.qualified_name)
            .expect("Type reference couldn't be resolved");

        tokens.append_all(
            syn::parse_str::<TokenStream>(
                &definition
                    .identifier
                    .reference_path(&self.context.dependencies),
            )
            .unwrap(),
        );
    }
}

#[derive(Clone, Debug)]
pub struct EnumReference<'a> {
    pub(crate) context: Context<'a>,
    pub(crate) qualified_name: &'a str,
}

impl<'a> ToTokens for EnumReference<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // Look up the declaration for the type being referenced.
        let definition = self
            .context
            .bundle
            .enum_definitions
            .iter()
            .find(|def| def.identifier.qualified_name == self.qualified_name)
            .expect("Enum reference couldn't be resolved");

        tokens.append_all(
            syn::parse_str::<TokenStream>(
                &definition
                    .identifier
                    .reference_path(&self.context.dependencies),
            )
            .unwrap(),
        );
    }
}
