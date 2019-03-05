// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub type StoreObjectIID = u64;
pub type StoreObjectOID = String;

pub enum StoreMetaKey {
    IIDIncr,
}

pub enum StoreMetaValue {
    IIDIncr(StoreObjectIID),
}

impl StoreMetaKey {
    pub fn as_u64(&self) -> u64 {
        match self {
            StoreMetaKey::IIDIncr => 0,
        }
    }
}
