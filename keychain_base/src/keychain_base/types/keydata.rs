use ic_cdk::export::candid::{CandidType, Deserialize};
use uuid::Uuid;

pub use crate::types::base_types::{KeyDataId, OwnerId};

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug)]
pub struct KeyDataCore {
    pub email_address: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}

pub struct KeyData {
    pub id: KeyDataId,
    //pub owner_id: OwnerId,
    pub email_address: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
}

impl KeyData {
    pub fn new(keydata_core: KeyDataCore, owner_id: &OwnerId) -> KeyData {
        let id = Uuid::new_v5(&Uuid::NAMESPACE_OID, owner_id.as_bytes());
        KeyData {
            id: id.to_string(),
            //owner_id: owner_id.clone(),
            email_address: keydata_core.email_address,
            phone_number: keydata_core.phone_number,
            address: keydata_core.address,
        }
    }

    pub fn copy_core(&self) -> KeyDataCore {
        KeyDataCore {
            email_address: self.email_address.clone(),
            phone_number: self.phone_number.clone(),
            address: self.address.clone(),
        }
    }

    pub fn change_core(&mut self, new_core: KeyDataCore) {
        if self.email_address != new_core.email_address {
            self.email_address = new_core.email_address;
        }

        if self.phone_number != new_core.phone_number {
            self.phone_number = new_core.phone_number;
        }

        if self.address != new_core.address {
            self.address = new_core.address;
        }
    }
}
