use ic_cdk::export::candid::{CandidType, Deserialize};
use uuid::Uuid;

pub use crate::types::base_types::{KeyDataId, ObfuscatedDataId, OwnerId};

#[derive(Clone, CandidType, Deserialize, PartialEq, Debug)]
pub struct ObfuscatedDataCore {
    pub website_url: String,
    pub website_id: Option<String>,
    pub obfuscated_email_address: String,
    pub obfuscated_phone_number: Option<String>,
    pub password: Option<String>,
}

pub type ObfuscatedDataCoreVec = Vec<ObfuscatedDataCore>;

pub struct ObfuscatedData {
    pub id: ObfuscatedDataId,
    pub keydata_id: KeyDataId,

    pub website_url: String,
    pub website_id: Option<String>,
    pub obfuscated_email_address: String,
    pub obfuscated_phone_number: Option<String>,
    pub password: Option<String>,
}

impl ObfuscatedData {
    pub fn new(
        obfuscated_data_core: ObfuscatedDataCore,
        keydata_id: &KeyDataId,
        owner_id: &OwnerId,
    ) -> ObfuscatedData {
        let string_to_hash = format!("{}-{}", &owner_id, &obfuscated_data_core.website_url);
        let id = Uuid::new_v5(&Uuid::NAMESPACE_X500, string_to_hash.as_bytes());

        ObfuscatedData {
            id: id.to_string(),
            keydata_id: keydata_id.clone(),
            website_url: obfuscated_data_core.website_url,
            website_id: obfuscated_data_core.website_id,
            obfuscated_email_address: obfuscated_data_core.obfuscated_email_address,
            obfuscated_phone_number: obfuscated_data_core.obfuscated_phone_number,
            password: obfuscated_data_core.password,
        }
    }

    pub fn copy_core(&self) -> ObfuscatedDataCore {
        ObfuscatedDataCore {
            website_url: self.website_url.clone(),
            website_id: self.website_id.clone(),
            obfuscated_email_address: self.obfuscated_email_address.clone(),
            obfuscated_phone_number: self.obfuscated_phone_number.clone(),
            password: self.password.clone(),
        }
    }
}
