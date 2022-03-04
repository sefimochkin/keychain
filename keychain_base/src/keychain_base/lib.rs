use ic_cdk_macros::*;

mod storage;
mod testing_utils;
mod types;
mod utils;

pub use crate::storage::*;
pub use crate::types::*;
pub use crate::utils::*;

#[update]
fn init_keydata(keydata_core: KeyDataCore, owner_id: OwnerId) -> Response<NoResponseData> {
    if let UnwrapOrThrowResult::Result(_) = get_keydata_id_or_throw(&owner_id) {
        return Response::Error(ErrorResponse{
            error_code: 409,
            error_message: format!("User with owner_id: {} has already been initialized. Use change_keydata to change key data.", owner_id),
        });
    }

    let keydata = KeyData::new(keydata_core, &owner_id);
    let keydata_id = keydata.id.clone();

    if let Some(error) = insert_keydata(keydata, &owner_id) {
        return Response::Error(error);
    }

    if let Some(error) = insert_owner_id_to_data_id(&keydata_id, &owner_id) {
        return Response::Error(error);
    }

    if let Some(error) = insert_owner_id_to_odid(&owner_id) {
        return Response::Error(error);
    }

    Response::Success(SuccessResponse {
        code: 201,
        data: None,
    })
}

#[update]
fn init_obfuscated_data(
    obfuscated_data_core: ObfuscatedDataCore,
    owner_id: OwnerId,
) -> Response<NoResponseData> {
    let keydata_id = match get_keydata_id_or_throw(&owner_id) {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    let obfuscated_data = ObfuscatedData::new(obfuscated_data_core, &keydata_id, &owner_id);
    let obfuscated_data_id = obfuscated_data.id.clone();
    let obfuscated_email_address = obfuscated_data.obfuscated_email_address.clone();

    if let Some(error) = insert_obfuscated_data(obfuscated_data, &owner_id) {
        return Response::Error(error);
    }

    if let Some(error) = insert_odid_into_odids_set(&obfuscated_data_id, &owner_id) {
        return Response::Error(error);
    }

    if let Some(error) =
        insert_email_address_to_odid(&obfuscated_email_address, &obfuscated_data_id, &owner_id)
    {
        return Response::Error(error);
    }

    Response::Success(SuccessResponse {
        code: 201,
        data: None,
    })
}

#[query]
fn get_keydata(owner_id: OwnerId) -> Response<KeyDataCore> {
    let keydata_id = match get_keydata_id_or_throw(&owner_id) {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    let keydata = match get_keydata_or_throw(keydata_id, &owner_id) {
        UnwrapOrThrowResult::Result(val) => val.copy_core(),
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    Response::Success(SuccessResponse {
        code: 200,
        data: Some(keydata),
    })
}

#[query]
fn get_obfuscated_data(owner_id: OwnerId) -> Response<ObfuscatedDataCoreVec> {
    if let UnwrapOrThrowResult::Error(err) = get_keydata_id_or_throw(&owner_id) {
        return Response::Error(err);
    }

    let obfuscated_data_ids = match get_odids_or_throw(&owner_id) {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    let mut obfuscated_data_cores = ObfuscatedDataCoreVec::with_capacity(obfuscated_data_ids.len());

    for odid in obfuscated_data_ids {
        let obfuscated_data_core = match get_obfuscated_data_or_throw(&odid, &owner_id) {
            UnwrapOrThrowResult::Result(val) => val.copy_core(),
            UnwrapOrThrowResult::Error(err) => return Response::Error(err),
        };
        obfuscated_data_cores.push(obfuscated_data_core);
    }

    Response::Success(SuccessResponse {
        code: 200,
        data: Some(obfuscated_data_cores),
    })
}

#[update]
fn change_keydata(keydata_core: KeyDataCore, owner_id: OwnerId) -> Response<NoResponseData> {
    let keydata_id = match get_keydata_id_or_throw(&owner_id) {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    let keydata = match get_mut_keydata_or_throw(keydata_id, &owner_id) {
        UnwrapOrThrowResult::Result(val) => val,
        UnwrapOrThrowResult::Error(err) => return Response::Error(err),
    };

    keydata.change_core(keydata_core);

    Response::Success(SuccessResponse {
        code: 201,
        data: None,
    })
}

#[cfg(test)]
mod tests {
    pub use self::testing_utils::*;
    use super::*;
    use itertools::izip;

    fn run_ok_init_keydata(keydata_core: &KeyDataCore, owner_id: &OwnerId) {
        let init_response = init_keydata(keydata_core.clone(), owner_id.clone());

        match init_response {
            Response::Success(response) => assert_eq!(response.code, 201),
            Response::Error(error_response) => panic!(
                "Got error during init_key: code: {}, message: {}",
                error_response.error_code, error_response.error_message
            ),
        }
    }

    fn run_ok_get_keydata(owner_id: &OwnerId) -> KeyDataCore {
        let get_response = get_keydata(owner_id.clone());

        match get_response {
            Response::Success(response) => {
                assert_eq!(response.code, 200);
                return response.data.unwrap();
            }
            Response::Error(error_response) => panic!(
                "Got error during get_keydata: code: {}, message: {}",
                error_response.error_code, error_response.error_message
            ),
        };
    }

    fn run_ok_get_obfuscated_data(owner_id: &OwnerId) -> ObfuscatedDataCoreVec {
        let get_response = get_obfuscated_data(owner_id.clone());

        match get_response {
            Response::Success(response) => {
                assert_eq!(response.code, 200);
                return response.data.unwrap();
            }
            Response::Error(error_response) => panic!(
                "Got error during get_obfuscated_data: code: {}, message: {}",
                error_response.error_code, error_response.error_message
            ),
        };
    }

    fn run_ok_change_keydata(keydata_core: &KeyDataCore, owner_id: &OwnerId) {
        let get_response = change_keydata(keydata_core.clone(), owner_id.clone());

        match get_response {
            Response::Success(response) => assert_eq!(response.code, 201),
            Response::Error(error_response) => panic!(
                "Got error during change_keydata: code: {}, message: {}",
                error_response.error_code, error_response.error_message
            ),
        };
    }

    #[test]
    fn test_simple_init_keydata() {
        let keydata_core = get_keydata_cores(1)[0].clone();
        let email_address = keydata_core.email_address.clone();
        let owner_id = get_owner_ids(1)[0].clone();
        run_ok_init_keydata(&keydata_core, &owner_id);

        let get_response = run_ok_get_keydata(&owner_id);

        assert_eq!(get_response.email_address, email_address);
        assert!(get_response.address.is_none());
        assert!(get_response.phone_number.is_none());
    }

    #[test]
    fn test_try_init_for_same_owner_twice() {
        let keydata_cores = get_keydata_cores(2);
        let first_keydata_core = keydata_cores[0].clone();
        let second_keydata_core = keydata_cores[1].clone();
        let email_address = first_keydata_core.email_address.clone();
        let owner_id = get_owner_ids(1)[0].clone();

        run_ok_init_keydata(&first_keydata_core, &owner_id);

        let init_response = init_keydata(second_keydata_core.clone(), owner_id.clone());

        match init_response {
            Response::Success(_) => panic!("Second init should have returned an error!"),
            Response::Error(error_response) => {
                assert_eq!(error_response.error_code, 409);
            }
        }

        let get_response = run_ok_get_keydata(&owner_id);

        assert_eq!(get_response.email_address, email_address);
        assert!(get_response.address.is_none());
        assert!(get_response.phone_number.is_none());
    }

    #[test]
    fn test_try_init_obfuscated_without_init_keydata() {
        let owner_id = get_owner_ids(1)[0].clone();

        let obfuscated_data_core = get_obfuscated_data_cores(1, false)[0].clone();
        let init_response = init_obfuscated_data(obfuscated_data_core, owner_id.clone());

        match init_response {
            Response::Success(_) => {
                panic!("Init without previous init_keydata should have returned an error!")
            }
            Response::Error(error_response) => {
                assert_eq!(error_response.error_code, 403);
            }
        }
    }

    #[test]
    fn test_try_get_obfuscated_data_without_init_keydata() {
        let owner_id = get_owner_ids(1)[0].clone();

        let get_response = get_obfuscated_data(owner_id.clone());

        match get_response {
            Response::Success(_) => panic!(
                "get_obfuscated_data without previous init_keydata should have returned an error!"
            ),
            Response::Error(error_response) => {
                assert_eq!(error_response.error_code, 403);
            }
        }
    }

    #[test]
    fn test_simple_init_obfuscated_data() {
        let keydata_core = get_keydata_cores(1)[0].clone();
        let owner_id = get_owner_ids(1)[0].clone();
        run_ok_init_keydata(&keydata_core, &owner_id);

        let obfuscated_data_core = get_obfuscated_data_cores(1, false)[0].clone();
        let website_url = obfuscated_data_core.website_url.clone();
        let obfuscated_email_address = obfuscated_data_core.obfuscated_email_address.clone();

        let init_response = init_obfuscated_data(obfuscated_data_core, owner_id.clone());
        match init_response {
            Response::Success(response) => assert_eq!(response.code, 201),
            Response::Error(error_response) => panic!(
                "Got error during init_obfuscated: code: {}, message: {}",
                error_response.error_code, error_response.error_message
            ),
        }

        let returned_obfuscated_data_cores = run_ok_get_obfuscated_data(&owner_id);

        assert_eq!(returned_obfuscated_data_cores.len(), 1);

        let returned_obfuscated_data_core = returned_obfuscated_data_cores[0].clone();
        assert_eq!(
            returned_obfuscated_data_core.website_url,
            website_url.clone()
        );
        assert!(returned_obfuscated_data_core.website_id.is_none());
        assert_eq!(
            returned_obfuscated_data_core.obfuscated_email_address,
            obfuscated_email_address.clone()
        );
        assert!(returned_obfuscated_data_core
            .obfuscated_phone_number
            .is_none());
        assert!(returned_obfuscated_data_core.password.is_none());
    }

    #[test]
    fn test_simple_get_empty_obfuscated_data() {
        let keydata_core = get_keydata_cores(1)[0].clone();
        let owner_id = get_owner_ids(1)[0].clone();
        run_ok_init_keydata(&keydata_core, &owner_id);

        let returned_obfuscated_data_cores = run_ok_get_obfuscated_data(&owner_id);

        assert_eq!(returned_obfuscated_data_cores.len(), 0);
    }

    #[test]
    fn test_simple_init_many_obfuscated_data() {
        let keydata_core = get_keydata_cores(1)[0].clone();
        let owner_id = get_owner_ids(1)[0].clone();
        run_ok_init_keydata(&keydata_core, &owner_id);

        let number_of_obfs = 5;
        let mut website_urls = Vec::with_capacity(number_of_obfs);
        let mut obfuscated_email_addresses = Vec::with_capacity(number_of_obfs);

        let obfuscated_data_cores = get_obfuscated_data_cores(number_of_obfs, false);
        for obfuscated_data_core in obfuscated_data_cores {
            let website_url = obfuscated_data_core.website_url.clone();
            website_urls.push(website_url);

            let obfuscated_email_address = obfuscated_data_core.obfuscated_email_address.clone();
            obfuscated_email_addresses.push(obfuscated_email_address);

            init_obfuscated_data(obfuscated_data_core, owner_id.clone());
        }

        let mut returned_obfuscated_data_cores = run_ok_get_obfuscated_data(&owner_id);
        assert_eq!(returned_obfuscated_data_cores.len(), number_of_obfs);

        returned_obfuscated_data_cores.sort_by_key(|v| v.website_url.clone());

        for (returned_obfuscated_data_core, website_url, obfuscated_email_address) in izip!(
            returned_obfuscated_data_cores,
            website_urls,
            obfuscated_email_addresses
        ) {
            assert_eq!(
                returned_obfuscated_data_core.website_url,
                website_url.clone()
            );
            assert!(returned_obfuscated_data_core.website_id.is_none());
            assert_eq!(
                returned_obfuscated_data_core.obfuscated_email_address,
                obfuscated_email_address.clone()
            );
            assert!(returned_obfuscated_data_core
                .obfuscated_phone_number
                .is_none());
            assert!(returned_obfuscated_data_core.password.is_none());
        }
    }

    #[test]
    fn test_try_change_keydata_without_init_keydata() {
        let owner_id = get_owner_ids(1)[0].clone();
        let keydata_core = get_keydata_cores(1)[0].clone();

        let get_response = change_keydata(keydata_core.clone(), owner_id.clone());
        match get_response {
            Response::Success(_) => panic!(
                "change_keydata without previous init_keydata should have returned an error!"
            ),
            Response::Error(error_response) => assert_eq!(error_response.error_code, 403),
        };
    }

    #[test]
    fn test_simple_change_keydata() {
        let owner_id = get_owner_ids(1)[0].clone();
        let keydata_cores = get_keydata_cores(2);
        let initial_keydata_core = keydata_cores[0].clone();

        run_ok_init_keydata(&initial_keydata_core, &owner_id);

        let second_keydata_core = keydata_cores[1].clone();
        let second_email_address = second_keydata_core.email_address.clone();

        run_ok_change_keydata(&second_keydata_core, &owner_id);

        let get_response = run_ok_get_keydata(&owner_id);

        assert_eq!(get_response.email_address, second_email_address);
        assert!(get_response.address.is_none());
        assert!(get_response.phone_number.is_none());
    }

    #[test]
    fn test_try_init_same_obfs_data_for_two_owners() {
        let keydata_cores = get_keydata_cores(2);
        let keydata_core_1 = keydata_cores[0].clone();
        let keydata_core_2 = keydata_cores[1].clone();

        let owner_id_1 = get_owner_ids(1)[0].clone();
        let owner_id_2 = get_owner_ids(1)[0].clone();

        run_ok_init_keydata(&keydata_core_1, &owner_id_1);
        run_ok_init_keydata(&keydata_core_2, &owner_id_2);

        let obfuscated_data_core = get_obfuscated_data_cores(1, false)[0].clone();

        init_obfuscated_data(obfuscated_data_core.clone(), owner_id_1.clone());
        let second_init_resposne =
            init_obfuscated_data(obfuscated_data_core.clone(), owner_id_2.clone());
        match second_init_resposne {
            Response::Success(_) => panic!(
                "Second init for the same obfuscated_email_address should have caused an error!"
            ),
            Response::Error(error_response) => assert_eq!(error_response.error_code, 409),
        }
    }
}
