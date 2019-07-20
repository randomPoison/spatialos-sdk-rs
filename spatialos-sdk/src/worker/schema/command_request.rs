use crate::worker::{
    component::{Component, ComponentId},
    commands::RequestData,
    schema::{owned::*, ArrayField, FieldId, Object, SchemaField, SchemaObjectType},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CommandRequest(PhantomData<*mut Schema_CommandRequest>);

impl CommandRequest {
    pub fn new<R, C>(request: &R) -> Owned<Self> where R: RequestData<C>, C: Component {
        let mut result: Owned<Self> = unsafe {
            Owned::new(Schema_CreateCommandRequest(C::ID, R::INDEX))
        };

        // Populate the command request
        request.into_object(result.fields_mut());

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe {
            Schema_GetCommandRequestComponentId(self.as_ptr())
        }
    }

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_CommandRequest) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_CommandRequest {
        self as *const _ as *mut _
    }

    pub(crate) fn fields(&self) -> &Object {
        unimplemented!()
    }

    pub(crate) fn fields_mut(&mut self) -> &mut Object {
        unimplemented!()
    }
}

impl OwnableImpl for CommandRequest {
    type Raw = Schema_CommandRequest;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyCommandRequest(inst);
    }
}

unsafe impl Send for CommandRequest {}
