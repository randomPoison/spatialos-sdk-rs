#![allow(non_camel_case_types)]

use proc_quote::{quote, ToTokens, TokenStreamExt};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

pub mod quotable;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    pub qualified_name: String,
    pub name: String,
    pub path: Vec<String>,
}

impl Identifier {
    /// Returns the subset of `path` that represents the package of the identifier.
    ///
    /// For example, if the qualified name of the item is "foo.bar.Baz", that would
    /// mean that the item is named `Baz` and is in the package `foo.bar`. This
    /// method would therefore return `["foo", "bar"]`.
    pub fn package_path(&self) -> &[String] {
        let first_non_lowercase = self
            .path
            .iter()
            .position(|seg| !seg.chars().next().unwrap().is_lowercase())
            .unwrap_or_else(|| self.path.len());
        &self.path[..first_non_lowercase]
    }

    /// Returns the path elements for the item, converting elements to `snake_case`
    /// as needed to ensure that they're valid Rust module identifiers.
    ///
    /// Does not include the name of the item itself. For example, if `path` is
    /// `["foo_foo", "BarBar", "bazBaz", "Quux"]`, the resuling iterator would yield
    /// "foo_foo", "bar_bar", "baz_baz". Note that each of the elements was
    /// converted to `snake_case` to adhere to Rusts style conventions for module
    /// names.
    pub fn module_path(&self) -> impl Iterator<Item = String> + '_ {
        let len = self.path.len() - 1;
        self.path[..len]
            .iter()
            .map(|seg| heck::SnekCase::to_snek_case(&**seg))
    }

    /// Returns the path to correctly reference the item by name.
    ///
    /// Note that the returned path is relative to the root for the generated code.
    pub fn reference_path(&self) -> proc_macro2::TokenStream {
        let path_string = self
            .module_path()
            .chain(std::iter::once(self.path[self.path.len() - 1].clone()))
            .collect::<Vec<_>>()
            .join("::");
        syn::parse_str(&path_string).unwrap()
    }
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

impl TypeReference {
    pub fn quotable<'a>(&'a self, bundle: &'a SchemaBundleV1) -> quotable::TypeReference<'a> {
        quotable::TypeReference {
            bundle,
            qualified_name: &*self.qualified_name,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumReference {
    pub qualified_name: String,
}

impl EnumReference {
    pub fn quotable<'a>(&'a self, bundle: &'a SchemaBundleV1) -> quotable::EnumReference<'a> {
        quotable::EnumReference {
            bundle,
            qualified_name: &*self.qualified_name,
        }
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

impl ValueTypeReference {
    pub fn quotable<'a>(&'a self, bundle: &'a SchemaBundleV1) -> quotable::ValueTypeReference<'a> {
        match self {
            ValueTypeReference::Primitive(prim) => quotable::ValueTypeReference::Primitive(*prim),
            ValueTypeReference::Enum(enum_ref) => {
                quotable::ValueTypeReference::Enum(enum_ref.quotable(bundle))
            }
            ValueTypeReference::Type(type_ref) => {
                quotable::ValueTypeReference::Type(type_ref.quotable(bundle))
            }
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
pub enum Value {
    #[serde(rename = "boolValue")]
    Bool(bool),

    #[serde(rename = "uint32Value")]
    U32(u32),

    #[serde(rename = "uint64Value")]
    U64(u64),

    #[serde(rename = "int32Value")]
    I32(i32),

    #[serde(rename = "int64Value")]
    I64(i64),

    #[serde(rename = "floatValue")]
    F32(f32),

    #[serde(rename = "doubleValue")]
    F64(f64),

    #[serde(rename = "stringValue")]
    String(String),

    #[serde(rename = "bytesValue")]
    Bytes(Vec<u8>),

    #[serde(rename = "entityIdValue")]
    EntityId(i64),

    #[serde(rename = "enumValue")]
    Enum(EnumValue),

    #[serde(rename = "typeValue")]
    Type(TypeValue),

    #[serde(rename = "optionValue")]
    Option(Value_OptionValue),

    #[serde(rename = "listValue")]
    List(Value_ListValue),

    #[serde(rename = "mapValue")]
    Map(Value_MapValue),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub enum_value: EnumValueReference,
    #[serde(rename = "enum")]
    pub enum_reference: TypeReference,
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

impl ToTokens for EnumValueDefinition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = syn::Ident::new(&self.identifier.name, proc_macro2::Span::call_site());
        let value = syn::LitInt::new(
            u64::from(self.value),
            syn::IntSuffix::None,
            proc_macro2::Span::call_site(),
        );
        tokens.append_all(quote! {
            #ident = #value
        });
    }
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

impl FieldTypeDefinition {
    pub fn quotable<'a>(&'a self, bundle: &'a SchemaBundleV1) -> quotable::FieldTypeDefinition<'a> {
        match self {
            FieldTypeDefinition::Singular { ty } => {
                quotable::FieldTypeDefinition::Singular(ty.quotable(bundle))
            }
            FieldTypeDefinition::Optional { inner_type } => {
                quotable::FieldTypeDefinition::Optional(inner_type.quotable(bundle))
            }
            FieldTypeDefinition::List { inner_type } => {
                quotable::FieldTypeDefinition::List(inner_type.quotable(bundle))
            }
            FieldTypeDefinition::Map {
                key_type,
                value_type,
            } => quotable::FieldTypeDefinition::Map {
                key: key_type.quotable(bundle),
                value: value_type.quotable(bundle),
            },
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
    #[serde(flatten)]
    pub data_definition: ComponentDataDefinition,
    pub event_definitions: Vec<ComponentDefinition_EventDefinition>,
    pub command_definitions: Vec<ComponentDefinition_CommandDefinition>,
    pub annotations: Vec<Annotation>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ComponentDataDefinition {
    #[serde(rename = "fieldDefinitions")]
    Inline(Vec<FieldDefinition>),

    #[serde(rename = "dataDefinition")]
    TypeReference(TypeReference),
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
