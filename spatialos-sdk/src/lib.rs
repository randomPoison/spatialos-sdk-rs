#![allow(non_upper_case_globals)]

extern crate spatialos_sdk_sys;

// TODO: Where should this live? We only need it for tests in order to reduce
// boilerplate, but it needs to live in the crate because we use it for both
// internal and external tests.
#[macro_export]
macro_rules! dummy_component {
    ($component:ident, $update:ident, $request:ident, $response:ident) => {
        impl $crate::worker::schema::SchemaObjectType for $component {
            fn from_object(_: &$crate::worker::schema::Object) -> Self {
                unimplemented!()
            }

            fn into_object(&self, _: &mut $crate::worker::schema::Object) {
                unimplemented!();
            }
        }

        impl $crate::worker::component::Component for $component {
            const ID: $crate::worker::component::ComponentId = 1234;
            type Update = $update;
            type Request = $request;
            type Response = $response;
        }

        inventory::submit!($crate::worker::component::VTable::new::<$component>());

        pub struct $update;

        impl $crate::worker::component::Update for $update {
            type Component = $component;

            fn from_update(_: &$crate::worker::schema::ComponentUpdate) -> Self {
                unimplemented!()
            }

            fn into_update(&self, _: &mut $crate::worker::schema::ComponentUpdate) {
                unimplemented!();
            }
        }

        pub struct $request;

        impl $crate::worker::commands::Request for $request {
            type Component = $component;

            fn into_request(
                &self,
            ) -> (
                $crate::worker::schema::owned::Owned<$crate::worker::schema::CommandRequest>,
                $crate::worker::commands::CommandIndex,
            ) {
                unimplemented!()
            }

            fn from_request(
                _: &$crate::worker::schema::CommandRequest,
                _: $crate::worker::commands::CommandIndex,
            ) -> Option<Self> {
                unimplemented!()
            }
        }

        pub struct $response;

        impl $crate::worker::commands::Response for $response {
            type Component = $component;

            fn into_response(
                &self,
            ) -> (
                $crate::worker::schema::owned::Owned<$crate::worker::schema::CommandResponse>,
                $crate::worker::commands::CommandIndex,
            ) {
                unimplemented!()
            }

            fn from_response(
                _: &$crate::worker::schema::CommandResponse,
                _: $crate::worker::commands::CommandIndex,
            ) -> Option<Self> {
                unimplemented!()
            }
        }
    };
}

pub(crate) mod ptr;
pub mod worker;
