use ic_cdk::storage;

pub use crate::types::base_types::*;
pub use crate::types::keydata::KeyData;
pub use crate::types::obfuscated_data::ObfuscatedData;
pub use crate::types::response_types::ErrorResponse;

pub use crate::storage::types::*;
pub use crate::utils::unwrappers::*;

pub fn get_keydata_id_or_throw<'a>(owner_id: &'a OwnerId) -> UnwrapOrThrowResult<&'a KeyDataId> {
    let owner_id_to_data_id = storage::get::<OwnerIdToDataId>();

    let opt_keydata_id = owner_id_to_data_id.get(owner_id);

    match opt_keydata_id {
        Some(val) => UnwrapOrThrowResult::Result(val),
        None => UnwrapOrThrowResult::Error(ErrorResponse {
            error_code: 403,
            error_message: format!("There is no user with owner_id: {}", owner_id),
        }),
    }
}

pub fn get_obfuscated_data_or_throw<'a>(
    odid: &ObfuscatedDataId,
    owner_id: &'a OwnerId,
) -> UnwrapOrThrowResult<&'a ObfuscatedData> {
    let obfuscated_data_store = storage::get::<ObfuscatedDataStore>();

    let obfuscated_data = unwrap_or_throw_500(
        obfuscated_data_store.get(odid),
        format!(
            "Could not find obfuscated data for odid: {}, owner_id: {}",
            &odid, &owner_id
        ),
        format!("Could not find obfuscated data for owner: {}", &owner_id),
    );
    obfuscated_data
}

pub fn get_odids_or_throw<'a>(
    owner_id: &'a OwnerId,
) -> UnwrapOrThrowResult<&'a ObfuscatedDataIdsSet> {
    let owner_id_to_obfuscated_data_ids = storage::get::<OwnerIdToObfuscatedDataIds>();

    let obfuscated_data_ids = unwrap_or_throw_500(
        owner_id_to_obfuscated_data_ids.get(owner_id),
        format!(
            "Could not find obfuscated_data ids for owner_id: {}",
            owner_id
        ),
        format!("Could not find obfuscated data for owner: {}", owner_id),
    );
    obfuscated_data_ids
}

pub fn get_mut_odids_or_throw<'a>(
    owner_id: &'a OwnerId,
) -> UnwrapOrThrowResult<&'a mut ObfuscatedDataIdsSet> {
    let owner_id_to_obfuscated_data_ids = storage::get_mut::<OwnerIdToObfuscatedDataIds>();

    let obfuscated_data_ids = unwrap_or_throw_500(
        owner_id_to_obfuscated_data_ids.get_mut(owner_id),
        format!(
            "Could not find obfuscated_data ids for owner_id: {}",
            owner_id
        ),
        format!("Could not find obfuscated data for owner: {}", owner_id),
    );
    obfuscated_data_ids
}

pub fn get_keydata_or_throw<'a>(
    keydata_id: &KeyDataId,
    owner_id: &'a OwnerId,
) -> UnwrapOrThrowResult<&'a KeyData> {
    let keydata_store = storage::get::<KeyDataStore>();

    let keydata = unwrap_or_throw_500(
        keydata_store.get(keydata_id),
        format!(
            "Could not find keydata with id: {}, owner_id: {}",
            keydata_id, owner_id
        ),
        format!("Could not find keydata for owner: {}", owner_id),
    );
    keydata
}

pub fn get_mut_keydata_or_throw<'a>(
    keydata_id: &KeyDataId,
    owner_id: &'a OwnerId,
) -> UnwrapOrThrowResult<&'a mut KeyData> {
    let keydata_store = storage::get_mut::<KeyDataStore>();

    let keydata = unwrap_or_throw_500(
        keydata_store.get_mut(keydata_id),
        format!(
            "Could not find keydata with id: {}, owner_id: {}",
            keydata_id, owner_id
        ),
        format!("Could not find keydata for owner: {}", owner_id),
    );
    keydata
}
