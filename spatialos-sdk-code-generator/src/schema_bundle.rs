#![allow(non_camel_case_types)]

use quote::{quote, ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub qualified_name: String,
    pub name: String,
    pub path: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum PrimitiveType {
    Invalid = 0,
    Int32 = 1,
    Int64 = 2,
    Uint32 = 3,
    Uint64 = 4,
    Sint32 = 5,
    Sint64 = 6,
    Fixed32 = 7,
    Fixed64 = 8,
    Sfixed32 = 9,
    Sfixed64 = 10,
    Bool = 11,
    Float = 12,
    Double = 13,
    String = 14,
    EntityId = 15,
    Bytes = 16,
}

impl ToTokens for PrimitiveType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = match self {
            PrimitiveType::Invalid => panic!("Invalid primitive type!"),
            PrimitiveType::Int32 | PrimitiveType::Sint32 | PrimitiveType::Sfixed32 => {
                quote! { i32 }
            }
            PrimitiveType::Int64 | PrimitiveType::Sint64 | PrimitiveType::Sfixed64 => {
                quote! { i64 }
            }
            PrimitiveType::Uint32 | PrimitiveType::Fixed32 => quote! { u32 },
            PrimitiveType::Uint64 | PrimitiveType::Fixed64 => quote! { u64 },
            PrimitiveType::Bool => quote! { bool },
            PrimitiveType::Float => quote! { f32 },
            PrimitiveType::Double => quote! { f64 },
            PrimitiveType::String => quote! { String },
            PrimitiveType::Bytes => quote! { Vec<u8> },

            // TODO: More robust handling of crate names/paths.
            PrimitiveType::EntityId => quote! { spatialos_sdk::worker::EntityId },
        };

        tokens.append_all(ty);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeReference {
    pub qualified_name: String,
}

impl ToTokens for TypeReference {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let path = self
            .qualified_name
            .split('.')
            .collect::<Vec<_>>()
            .join("::");
        tokens.append_all(syn::parse_str::<proc_macro2::TokenStream>(&path));
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumReference {
    pub qualified_name: String,
}

impl ToTokens for EnumReference {
    fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
        unimplemented!();
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueReference {
    pub qualified_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldReference {
    pub qualified_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ValueTypeReference {
    Primitive(PrimitiveType),
    Enum(EnumReference),
    Type(TypeReference),
}

impl ToTokens for ValueTypeReference {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ValueTypeReference::Primitive(primitive) => primitive.to_tokens(tokens),
            ValueTypeReference::Enum(enum_ty) => enum_ty.to_tokens(tokens),
            ValueTypeReference::Type(ty) => ty.to_tokens(tokens),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_OptionValue {
    pub value: Option<Box<Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_ListValue {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue_MapPairValue {
    pub key: Value,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value_MapValue {
    pub values: Vec<Value_MapValue_MapPairValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub bool_value: Option<bool>,
    pub uint32_value: Option<u32>,
    pub uint64_value: Option<u64>,
    pub int32_value: Option<i32>,
    pub int64_value: Option<i64>,
    pub float_value: Option<f32>,
    pub double_value: Option<f64>,
    pub string_value: Option<String>,
    pub bytes_value: Option<String>,
    pub entity_id_value: Option<i64>,
    pub enum_value: Option<EnumValue>,
    pub type_value: Option<TypeValue>,
    pub option_value: Option<Value_OptionValue>,
    pub list_value: Option<Value_ListValue>,
    pub map_value: Option<Value_MapValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub enum_value: EnumValueReference,
    #[serde(rename = "enum")]
    pub enum_reference: EnumReference,
    pub name: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeValue_FieldValue {
    pub field: FieldReference,
    pub name: String,
    pub number: u32,
    pub value: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeValue {
    #[serde(rename = "type")]
    pub type_reference: TypeReference,
    pub fields: Vec<TypeValue_FieldValue>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub type_value: TypeValue,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDefinition {
    pub identifier: Identifier,
    pub value: u32,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub identifier: Identifier,
    pub value_definitions: Vec<EnumValueDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    pub identifier: Identifier,
    pub field_id: u32,
    pub transient: bool,
    #[serde(flatten)]
    pub ty: FieldTypeDefinition,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FieldTypeDefinition {
    #[serde(rename = "singularType")]
    #[serde(rename_all = "camelCase")]
    Singular {
        #[serde(rename = "type")]
        ty: ValueTypeReference,
    },

    #[serde(rename = "optionType")]
    #[serde(rename_all = "camelCase")]
    Optional { inner_type: ValueTypeReference },

    #[serde(rename = "listType")]
    #[serde(rename_all = "camelCase")]
    List { inner_type: ValueTypeReference },

    #[serde(rename = "mapType")]
    #[serde(rename_all = "camelCase")]
    Map {
        key_type: ValueTypeReference,
        value_type: ValueTypeReference,
    },
}

impl ToTokens for FieldTypeDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldTypeDefinition::Singular { ty } => {
                ty.to_tokens(tokens);
            }

            FieldTypeDefinition::Optional { inner_type } => {
                tokens.append_all(quote! {
                    Option<#inner_type>
                });
            }

            FieldTypeDefinition::List { inner_type } => {
                tokens.append_all(quote! {
                    List<#inner_type>
                });
            }

            FieldTypeDefinition::Map {
                key_type,
                value_type,
            } => {
                tokens.append_all(quote! {
                    std::collections::BTreeMap<#key_type, #value_type>
                });
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinition {
    pub identifier: Identifier,
    pub field_definitions: Vec<FieldDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_EventDefinition {
    pub identifier: Identifier,
    pub event_index: u32,
    #[serde(rename = "type")]
    pub type_reference: ValueTypeReference,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition_CommandDefinition {
    pub identifier: Identifier,
    pub command_index: u32,
    pub request_type: ValueTypeReference,
    pub response_type: ValueTypeReference,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComponentDefinition {
    pub identifier: Identifier,
    pub component_id: u32,
    pub data_definition: Option<TypeReference>,
    pub field_definitions: Vec<FieldDefinition>,
    pub event_definitions: Vec<ComponentDefinition_EventDefinition>,
    pub command_definitions: Vec<ComponentDefinition_CommandDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundleV1 {
    pub enum_definitions: Vec<EnumDefinition>,
    pub type_definitions: Vec<TypeDefinition>,
    pub component_definitions: Vec<ComponentDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SourceReference {
    pub file_path: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSourceMapV1 {
    pub source_references: HashMap<String, SourceReference>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBundle {
    pub v1: Option<SchemaBundleV1>,
    pub source_map_v1: Option<SchemaSourceMapV1>,
}

pub fn load_bundle(data: &str) -> Result<SchemaBundle, serde_json::Error> {
    serde_json::from_str::<SchemaBundle>(data)
}
