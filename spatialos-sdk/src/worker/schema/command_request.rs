use crate::worker::{
    commands::CommandIndex,
    component::{Component, ComponentId},
    schema::{owned::*, Object, SchemaObjectType},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CommandRequest(PhantomData<*mut Schema_CommandRequest>);

impl CommandRequest {
    pub fn new<C: Component, R: SchemaObjectType>(index: CommandIndex, request: &R) -> Owned<Self> {
        let mut result: Owned<Self> =
            unsafe { Owned::new(Schema_CreateCommandRequest(C::ID, index)) };

        // Populate the command request.
        request.into_object(result.fields_mut());

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetCommandRequestComponentId(self.as_ptr()) }
    }

    pub fn index(&self) -> CommandIndex {
        unsafe { Schema_GetCommandRequestCommandIndex(self.as_ptr()) }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_CommandRequest) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_CommandRequest {
        self as *const _ as *mut _
    }

    pub fn fields(&self) -> &Object {
        unsafe {
            Object::from_raw(Schema_GetCommandRequestObject(self.as_ptr()))
        }
    }

    pub fn fields_mut(&mut self) -> &mut Object {
        unsafe {
            Object::from_raw_mut(Schema_GetCommandRequestObject(self.as_ptr()))
        }
    }
}

impl OwnableImpl for CommandRequest {
    type Raw = Schema_CommandRequest;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyCommandRequest(inst);
    }
}

unsafe impl Send for CommandRequest {}
