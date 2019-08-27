use crate::worker::{
    component::Component,
    schema::{owned::*, Object, SchemaObjectType},
};
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CommandResponse(PhantomData<*mut Schema_CommandResponse>);

impl CommandResponse {
    pub fn new<C: Component, R: SchemaObjectType>(response: &R) -> Owned<Self> {
        let mut result: Owned<Self> = unsafe { Owned::new(Schema_CreateCommandResponse()) };

        // Populate the command response.
        response.into_object(result.fields_mut());

        result
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }

    pub(crate) unsafe fn from_raw<'a>(raw: *mut Schema_CommandResponse) -> &'a Self {
        &*(raw as *mut _)
    }

    pub(crate) fn as_ptr(&self) -> *mut Schema_CommandResponse {
        self as *const _ as *mut _
    }

    pub fn fields(&self) -> &Object {
        unsafe { Object::from_raw(Schema_GetCommandResponseObject(self.as_ptr())) }
    }

    pub fn fields_mut(&mut self) -> &mut Object {
        unsafe { Object::from_raw_mut(Schema_GetCommandResponseObject(self.as_ptr())) }
    }
}

impl OwnableImpl for CommandResponse {
    type Raw = Schema_CommandResponse;

    unsafe fn destroy(inst: *mut Self::Raw) {
        Schema_DestroyCommandResponse(inst);
    }
}

unsafe impl Send for CommandResponse {}
