//! Example code for the following schema defintions:
//!
//! ```schema
//! component CustomComponent {
//!     id = 1234;
//!
//!     option<string> name = 1;
//!     int32 count = 2;
//!     list<EntityId> targets = 3;
//!     list<bytes> byte_collection = 4;
//!     option<uint32> id = 5;
//!     NestedType nested = 6;
//!     list<bool> events = 7;
//!
//!     event NestedType nested_event;
//!     event CoolEvent cool_event;
//!
//!     command CoolEvent some_command(NestedType);
//!     command CoolEvent duplicate_command(NestedType);
//!     command NestedType other_command(CoolEvent);
//! }
//!
//! type NestedType {
//!     string name = 1;
//! }
//!
//! type CoolEvent {
//!     option<bytes> some_data = 1;
//! }
//! ```

use spatialos_sdk::worker::{
    component::*,
    commands::*,
    schema::{self, *, owned::Owned},
    EntityId,
};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct CustomComponent {
    pub name: Option<String>,
    pub count: i32,
    pub targets: Vec<EntityId>,
    pub target_names: BTreeMap<EntityId, String>,
    pub byte_collection: Vec<Vec<u8>>,
    pub id: Option<u32>,
    pub nested: NestedType,
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &schema::Object) -> Self {
        Self {
            name: object.field::<Option<String>>(0),
            count: object.field::<SchemaSfixed32>(1),
            targets: object.field_array::<EntityId>(2),
            target_names: object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: object.field::<Vec<Vec<u8>>>(4),
            id: object.field::<Option<SchemaUint32>>(5),
            nested: object.field::<NestedType>(6),
        }
    }

    fn into_object(&self, object: &mut schema::Object) {
        object.add_field::<Option<String>>(0, &self.name);
        object.add_field::<SchemaSfixed32>(1, &self.count);
        object.add_field_array::<EntityId>(2, &self.targets);
        object.add_field::<BTreeMap<EntityId, String>>(3, &self.target_names);
        object.add_field::<Vec<Vec<u8>>>(4, &self.byte_collection);
        object.add_field::<Option<SchemaUint32>>(5, &self.id);
        object.add_field::<NestedType>(6, &self.nested);
    }
}

impl Component for CustomComponent {
    const ID: ComponentId = 1234;
    type Update = CustomComponentUpdate;
    type Request = CustomComponentCommandRequest;
    type Response = CustomComponentCommandResponse;
}

#[allow(clippy::option_option)]
pub struct CustomComponentUpdate {
    pub name: Option<Option<String>>,
    pub count: Option<i32>,
    pub targets: Option<Vec<EntityId>>,
    pub target_names: Option<BTreeMap<EntityId, String>>,
    pub byte_collection: Option<Vec<Vec<u8>>>,
    pub id: Option<Option<u32>>,
    pub nested: Option<NestedType>,

    pub nested_event: Vec<NestedType>,
    pub cool_event: Vec<CoolEvent>,
}

impl Update for CustomComponentUpdate {
    type Component = CustomComponent;

    fn from_update(update: &schema::ComponentUpdate) -> Self {
        Self {
            name: update.field::<Option<String>>(0),
            count: update.field::<SchemaSfixed32>(1),
            targets: update.field_array::<EntityId>(2),
            target_names: update.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: update.field::<Vec<Vec<u8>>>(4),
            id: update.field::<Option<SchemaUint32>>(5),
            nested: update.field::<NestedType>(6),

            nested_event: update.event::<NestedType>(1),
            cool_event: update.event::<CoolEvent>(2),
        }
    }

    fn into_update(&self, update: &mut schema::ComponentUpdate) {
        if let Some(name) = &self.name {
            update.add_field::<Option<String>>(0, name);
        }

        if let Some(count) = &self.count {
            update.add_field::<SchemaSfixed32>(1, count);
        }

        if let Some(target) = &self.targets {
            update.add_field_array::<EntityId>(2, target);
        }

        if let Some(target_names) = &self.target_names {
            update.add_field::<BTreeMap<EntityId, String>>(3, target_names);
        }

        if let Some(byte_collection) = &self.byte_collection {
            update.add_field::<Vec<Vec<u8>>>(4, byte_collection);
        }

        if let Some(id) = &self.id {
            update.add_field::<Option<SchemaUint32>>(5, id);
        }

        if let Some(nested) = &self.nested {
            update.add_field::<NestedType>(6, nested);
        }

        if !self.nested_event.is_empty() {
            update.add_event(1, &self.nested_event);
        }

        if !self.cool_event.is_empty() {
            update.add_event(2, &self.cool_event);
        }
    }
}

pub enum CustomComponentCommandRequest {
    SomeCommand(NestedType),
    DuplicateCommand(NestedType),
    OtherCommand(CoolEvent),
}

impl Request for CustomComponentCommandRequest {
    type Component = CustomComponent;

    fn into_request(&self) -> (Owned<CommandRequest>, CommandIndex) {
        match self {
            CustomComponentCommandRequest::SomeCommand(request) =>
                (CommandRequest::new::<Self::Component, _>(request), 1),

            CustomComponentCommandRequest::DuplicateCommand(request) =>
                (CommandRequest::new::<Self::Component, _>(request), 2),

            CustomComponentCommandRequest::OtherCommand(request) =>
                (CommandRequest::new::<Self::Component, _>(request), 3),
        }
    }

    fn from_request(request: &CommandRequest, index: CommandIndex) -> Option<Self> {
        match index {
            1 => Some(CustomComponentCommandRequest::SomeCommand(request.deserialize())),
            2 => Some(CustomComponentCommandRequest::DuplicateCommand(request.deserialize())),
            3 => Some(CustomComponentCommandRequest::OtherCommand(request.deserialize())),

            _ => None,
        }
    }
}

pub enum CustomComponentCommandResponse {
    SomeCommand(CoolEvent),
    DuplicateCommand(CoolEvent),
    OtherCommand(NestedType),
}

impl Response for CustomComponentCommandResponse {
    type Component = CustomComponent;

    fn into_response(&self) -> (Owned<CommandResponse>, CommandIndex) {
        match self {
            CustomComponentCommandResponse::SomeCommand(response) =>
                (CommandResponse::new::<Self::Component, _>(response), 1),

            CustomComponentCommandResponse::DuplicateCommand(response) =>
                (CommandResponse::new::<Self::Component, _>(response), 2),

            CustomComponentCommandResponse::OtherCommand(response) =>
                (CommandResponse::new::<Self::Component, _>(response), 3),
        }
    }

    fn from_response(response: &CommandResponse, index: CommandIndex) -> Option<Self> {
        match index {
            1 => Some(CustomComponentCommandResponse::SomeCommand(response.deserialize())),
            2 => Some(CustomComponentCommandResponse::DuplicateCommand(response.deserialize())),
            3 => Some(CustomComponentCommandResponse::OtherCommand(response.deserialize())),

            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct NestedType {
    pub name: String,
}

impl SchemaObjectType for NestedType {
    fn from_object(_object: &schema::Object) -> Self {
        unimplemented!()
    }

    fn into_object(&self, _object: &mut schema::Object) {
        unimplemented!();
    }
}

#[derive(Debug, Default)]
pub struct CoolEvent {
    pub some_data: Option<Vec<u8>>,
}

impl SchemaObjectType for CoolEvent {
    fn from_object(_object: &schema::Object) -> Self {
        unimplemented!()
    }

    fn into_object(&self, _object: &mut schema::Object) {
        unimplemented!();
    }
}

fn main() {}
