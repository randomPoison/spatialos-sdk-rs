pub mod commands;
pub mod component;
pub mod entity_snapshot;
pub mod metrics;
pub mod op;

use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EntityId {
    pub id: i64,
}

impl EntityId {
    pub fn new(id: i64) -> EntityId {
        EntityId { id }
    }

    pub fn is_valid(&self) -> bool {
        self.id > 0
    }

    pub fn to_string(&self) -> String {
        format!("EntityId: {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RequestId<T> {
    id: u32,
    _type: PhantomData<*const T>,
}

impl<T> RequestId<T> {
    pub fn new(id: u32) -> RequestId<T> {
        RequestId {
            id,
            _type: PhantomData,
        }
    }

    pub fn to_string(&self) -> String {
        format!("RequestId: {}", self.id)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub enum Authority {
    Authoritative(),
    AuthorityLossImminent(),
    NotAuthoritative(),
}

impl Authority {
    pub fn has_authority(&self) -> bool {
        self != &Authority::NotAuthoritative()
    }
}

impl From<u8> for Authority {
    fn from(auth: u8) -> Self {
        match auth {
            0 => Authority::NotAuthoritative(),
            1 => Authority::Authoritative(),
            2 => Authority::AuthorityLossImminent(),
            _ => panic!("Unknown authority state: {}", auth),
        }
    }
}
