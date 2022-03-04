use ic_cdk::storage;

pub use crate::types::base_types::*;
pub use crate::types::keydata::KeyData;
pub use crate::types::obfuscated_data::ObfuscatedData;
pub use crate::types::response_types::ErrorResponse;

pub use crate::getters::get_mut_odids_or_throw;
pub use crate::storage::types::*;
pub use crate::utils::unwrappers::*;

fn insert_or_throw<S, I: FnOnce(&mut S) -> (), L: Fn(&mut S) -> usize>(
    storage: &mut S,
    inserter: I,
    len: L,
    debug_message: String,
    response_message: String,
) -> Option<ErrorResponse> {
    let size_pre = len(storage);

    inserter(storage);

    if size_pre == len(storage) {
        eprintln!("{}", &debug_message);
        return Some(ErrorResponse {
            error_code: 409,
            error_message: response_message,
        });
    }

    None
}

pub fn insert_keydata(keydata: KeyData, owner_id: &OwnerId) -> Option<ErrorResponse> {
    let keydata_store = storage::get_mut::<KeyDataStore>();
    let keydata_id = keydata.id.clone();
    if let Some(error) = insert_or_throw(
        keydata_store,
        |storage| {
            storage.insert(keydata.id.clone(), keydata);
        },
        |storage| storage.len(),
        format!(
            "Could not insert keydata! keydata_id: {}, owner_id: {}",
            keydata_id, owner_id
        ),
        format!("Could not create keydata for owner: {}", owner_id),
    ) {
        return Some(error);
    }

    None
}

pub fn insert_obfuscated_data(
    obfuscated_data: ObfuscatedData,
    owner_id: &OwnerId,
) -> Option<ErrorResponse> {
    let obfuscated_data_store = storage::get_mut::<ObfuscatedDataStore>();
    let obfuscated_data_id = obfuscated_data.id.clone();
    if let Some(error) = insert_or_throw(
        obfuscated_data_store,
        |storage| {
            storage.insert(obfuscated_data.id.clone(), obfuscated_data);
        },
        |storage| storage.len(),
        format!(
            "Could not insert obfuscated data! odid: {}, owner_id: {}",
            obfuscated_data_id, owner_id
        ),
        format!("Could not create obfuscated data for owner: {}", owner_id),
    ) {
        return Some(error);
    }

    None
}

pub fn insert_owner_id_to_data_id(
    keydata_id: &KeyDataId,
    owner_id: &OwnerId,
) -> Option<ErrorResponse> {
    let owner_id_to_data_id = storage::get_mut::<OwnerIdToDataId>();

    if let Some(error) = insert_or_throw(
        owner_id_to_data_id,
        |storage| {
            storage.insert(owner_id.clone(), keydata_id.clone());
        },
        |storage| storage.len(),
        format!(
            "Could not insert owner_id->keydata_id! keydata_id: {}, owner_id: {}",
            keydata_id.clone(),
            owner_id
        ),
        format!(
            "Could not create link from owner to keydata for owner: {}",
            owner_id
        ),
    ) {
        return Some(error);
    }

    None
}

pub fn insert_owner_id_to_odid(owner_id: &OwnerId) -> Option<ErrorResponse> {
    let owner_id_to_obfuscated_data_ids = storage::get_mut::<OwnerIdToObfuscatedDataIds>();

    if let Some(error) = insert_or_throw(
        owner_id_to_obfuscated_data_ids,
        |storage| {
            storage.insert(owner_id.clone(), ObfuscatedDataIdsSet::new());
        },
        |storage| storage.len(),
        format!("Could not insert owner_id->odids! owner_id: {}", owner_id),
        format!(
            "Could not create link from owner to obfuscated data for owner: {}",
            owner_id
        ),
    ) {
        return Some(error);
    }

    None
}

pub fn insert_odid_into_odids_set(
    obfuscated_data_id: &ObfuscatedDataId,
    owner_id: &OwnerId,
) -> Option<ErrorResponse> {
    let obfuscated_data_ids = get_mut_odids_or_throw(&owner_id);
    let obfuscated_data_ids = match obfuscated_data_ids {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Some(err),
    };

    if let Some(error) = insert_or_throw(
        obfuscated_data_ids,
        |storage| {
            storage.insert(obfuscated_data_id.clone());
        },
        |storage| storage.len(),
        format!(
            "Could not insert odid into odids set! odid: {}, owner_id: {}",
            &obfuscated_data_id, owner_id
        ),
        format!(
            "Could not create link from owner to obfuscated data for owner: {}",
            owner_id
        ),
    ) {
        return Some(error);
    }

    None
}

pub fn insert_email_address_to_odid(
    obfuscated_email_address: &String,
    obfuscated_data_id: &ObfuscatedDataId,
    owner_id: &OwnerId,
) -> Option<ErrorResponse> {
    let obfuscated_email_address_to_odid =
        storage::get_mut::<ObfuscatedEmailAddressToObfuscatedDataId>();

    if let Some(error) = insert_or_throw(
        obfuscated_email_address_to_odid,
        |storage| {storage.insert(obfuscated_email_address.clone(), obfuscated_data_id.clone());},
        |storage| {storage.len()},
        format!("Could not insert obfuscated_email_address->odid! obfuscated_email_address: {}, owner_id: {}", &obfuscated_email_address, owner_id),
        format!("Could not create link from obfuscated email address to odid for obfuscated email address: {}, owner: {}", &obfuscated_email_address, owner_id),
    ) {
        return Some(error);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    pub use crate::testing_utils::*;

    #[test]
    fn test_insert_keydata_twice() {
        let owner_id = get_owner_ids(1)[0].clone();
        let keydata_core = get_keydata_cores(1)[0].clone();

        let keydata_one = KeyData::new(keydata_core.clone(), &owner_id);
        let keydata_two = KeyData::new(keydata_core.clone(), &owner_id);

        insert_keydata(keydata_one, &owner_id);
        match insert_keydata(keydata_two, &owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => panic!("Inserting the same keydata twice should have caused an error!"),
        }
    }

    #[test]
    fn test_insert_obfuscated_data_twice() {
        let owner_id = get_owner_ids(1)[0].clone();
        let keydata_core = get_keydata_cores(1)[0].clone();
        let keydata = KeyData::new(keydata_core.clone(), &owner_id);

        let obfuscated_data_core = get_obfuscated_data_cores(1, false)[0].clone();
        let obfuscated_data_one =
            ObfuscatedData::new(obfuscated_data_core.clone(), &keydata.id, &owner_id);
        let obfuscated_data_two =
            ObfuscatedData::new(obfuscated_data_core.clone(), &keydata.id, &owner_id);

        insert_obfuscated_data(obfuscated_data_one, &owner_id);
        match insert_obfuscated_data(obfuscated_data_two, &owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => panic!("Inserting the same obfuscated_data twice should have caused an error!"),
        }
    }

    #[test]
    fn test_insert_owner_id_to_keydata_id_twice() {
        let ids = get_owner_ids(2);
        let owner_id = ids[0].clone();
        let keydata_id = ids[1].clone();

        insert_owner_id_to_data_id(&keydata_id, &owner_id);
        match insert_owner_id_to_data_id(&keydata_id, &owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => {
                panic!("Inserting the same owner_id->keydata_id twice should have caused an error!")
            }
        }
    }

    #[test]
    fn test_insert_owner_id_to_odids_twice() {
        let owner_id = get_owner_ids(1)[0].clone();

        insert_owner_id_to_odid(&owner_id);
        match insert_owner_id_to_odid(&owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => panic!("Inserting the same owner_id->odids twice should have caused an error!"),
        }
    }

    #[test]
    fn test_insert_odid_into_odids_twice() {
        let ids = get_owner_ids(2);
        let owner_id = ids[0].clone();
        let odid = ids[1].clone();

        insert_owner_id_to_odid(&owner_id);
        insert_odid_into_odids_set(&odid, &owner_id);
        match insert_odid_into_odids_set(&odid, &owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => {
                panic!("Inserting the same odid int odids_set twice should have caused an error!")
            }
        }
    }

    #[test]
    fn test_insert_email_address_to_odid_twice() {
        let ids = get_owner_ids(2);
        let owner_id = ids[0].clone();
        let odid = ids[1].clone();
        let email_address = String::from("email_address");

        insert_email_address_to_odid(&email_address, &odid, &owner_id);
        match insert_email_address_to_odid(&email_address, &odid, &owner_id) {
            Some(err) => assert_eq!(err.error_code, 409),
            None => {
                panic!("Inserting the same email_address->odid twice should have caused an error!")
            }
        }
    }
}
