use std::collections::HashMap;
use std::collections::HashSet;

pub use crate::types::base_types::*;
pub use crate::types::keydata::*;
pub use crate::types::obfuscated_data::*;

pub type KeyDataStore = HashMap<KeyDataId, KeyData>;
pub type ObfuscatedDataStore = HashMap<ObfuscatedDataId, ObfuscatedData>;
pub type ObfuscatedDataIdsSet = HashSet<ObfuscatedDataId>;

pub type OwnerIdToDataId = HashMap<OwnerId, KeyDataId>;
pub type OwnerIdToObfuscatedDataIds = HashMap<OwnerId, ObfuscatedDataIdsSet>;
pub type ObfuscatedEmailAddressToObfuscatedDataId = HashMap<String, ObfuscatedDataId>;
