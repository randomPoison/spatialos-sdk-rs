#![allow(non_camel_case_types)]

use crate::Context;
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
    pub fn reference_path(&self, dependencies: &HashMap<&'static str, &'static str>) -> String {
        dbg!(self);

        // Find the dependency package that this identifier belongs to.
        //
        // NOTE: We need to do a linear search here because there's no way of knowing
        // from the identifier path alone which portion of the path represents the
        // package name (so we can't do a direct map lookup). We need to check all of
        // the keys in the map to see which one is a prefix of the fully-qualified
        // identifier.
        //
        // NOTE: We do the extra fold step (instead of just returning the first match)
        // in case there are multiple packages with overlapping names, e.g. `foo` and
        // `foo.bar`. If we were to use the first matching prefix that we find, it's
        // possible that the identifier will be treated as belonging to `foo` when it
        // really belongs to `foo.bar`. We need to instead find the *longest* valid
        // prefix, which is handling by folding all matching prefixes and returning the
        // longest one.
        let (_, path_prefix) = dbg!(dependencies)
            .iter()
            .filter(|&(key, _)| self.qualified_name.starts_with(key))
            .fold(None, |longest, (&key, &value)| match longest {
                None => Some((key, value)),
                Some((longest_key, longest_value)) => {
                    if key.len() > longest_key.len() {
                        Some((key, value))
                    } else {
                        Some((longest_key, longest_value))
                    }
                }
            })
            .unwrap_or_else(|| panic!("No dependency definition found for {}, make sure you have specified all dependencies", self.qualified_name));

        let path_suffix = self
            .module_path()
            .chain(std::iter::once(self.path[self.path.len() - 1].clone()))
            .collect::<Vec<_>>()
            .join("::");
        format!("{}::{}", path_prefix, path_suffix)
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
    pub fn quotable<'a>(&'a self, context: Context<'a>) -> quotable::TypeReference<'a> {
        quotable::TypeReference {
            context,
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
    pub fn quotable<'a>(&'a self, context: Context<'a>) -> quotable::EnumReference<'a> {
        quotable::EnumReference {
            context,
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
    pub fn quotable<'a>(&'a self, context: Context<'a>) -> quotable::ValueTypeReference<'a> {
        match self {
            ValueTypeReference::Primitive(prim) => quotable::ValueTypeReference::Primitive(*prim),
            ValueTypeReference::Enum(enum_ref) => {
                quotable::ValueTypeReference::Enum(enum_ref.quotable(context))
            }
            ValueTypeReference::Type(type_ref) => {
                quotable::ValueTypeReference::Type(type_ref.quotable(context))
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
    pub fn quotable<'a>(&'a self, context: Context<'a>) -> quotable::FieldTypeDefinition<'a> {
        match self {
            FieldTypeDefinition::Singular { ty } => {
                quotable::FieldTypeDefinition::Singular(ty.quotable(context))
            }
            FieldTypeDefinition::Optional { inner_type } => {
                quotable::FieldTypeDefinition::Optional(inner_type.quotable(context))
            }
            FieldTypeDefinition::List { inner_type } => {
                quotable::FieldTypeDefinition::List(inner_type.quotable(context))
            }
            FieldTypeDefinition::Map {
                key_type,
                value_type,
            } => quotable::FieldTypeDefinition::Map {
                key: key_type.quotable(context),
                value: value_type.quotable(context),
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

impl SchemaBundleV1 {
    pub fn get_referenced_type(&self, type_ref: &TypeReference) -> &TypeDefinition {
        self.type_definitions
            .iter()
            .find(|type_def| type_def.identifier.qualified_name == type_ref.qualified_name)
            .unwrap_or_else(|| panic!("Cannot find type for reference {}", type_ref.qualified_name))
    }
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

#[cfg(test)]
mod tests {
    /// Tests that `Identifier::reference_path` correctly handles nested package
    /// names, e.g. that if packages `foo` and `foo.bar` are both listed as
    /// dependencies, it'll correctly identify `foo.bar.SomeType` as being part of
    /// package `foo.bar`.
    #[test]
    fn reference_path_construction() {
        use maplit::hashmap;

        let identifier = crate::schema_bundle::Identifier {
            qualified_name: "foo.bar.baz.quux.SomeType".into(),
            name: "SomeType".into(),
            path: vec![
                "foo".into(),
                "bar".into(),
                "baz".into(),
                "quux".into(),
                "SomeType".into(),
            ],
        };

        let dependencies = hashmap! {
            "foo" => "foo::schema",
            "foo.bar" => "bar::schema",
            "foo.bar.baz" => "baz::schema",
            "foo.bar.baz.quux" => "quux::schema",
        };

        let result = identifier.reference_path(&dependencies);
        assert_eq!(
            result.to_string(),
            "quux::schema::foo::bar::baz::quux::SomeType"
        );
    }
}
