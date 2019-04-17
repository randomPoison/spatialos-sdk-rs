extern crate spatialos_sdk;

use spatialos_sdk::worker::{schema::*, EntityId};

pub struct CustomComponent {
    pub targets: Vec<EntityId>,
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &SchemaObject) -> Self {
        Self {
            targets: object.field_array::<EntityId>(0),
        }
    }

    fn into_object<'a>(&'a self, object: &mut SchemaObject<'a>) {
        let bad_targets = Vec::new();
        object.add_field_array::<EntityId>(0, &bad_targets);
        //~^ ERROR does not live long enough
    }
}

fn main() {}