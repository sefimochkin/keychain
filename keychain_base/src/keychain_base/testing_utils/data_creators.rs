use rand::Rng;

pub use crate::types::*;

pub fn get_owner_ids(number: usize) -> Vec<OwnerId> {
    let rand_base = 100000;
    let mut rng = rand::thread_rng();

    let mut owner_ids = Vec::with_capacity(number);

    for i in 0..number {
        let owner_id = format!("owner_id_{}", i * rand_base + rng.gen_range(0..rand_base));
        owner_ids.push(owner_id);
    }
    owner_ids
}

pub fn get_keydata_cores(number: usize) -> Vec<KeyDataCore> {
    let mut keydata_cores = Vec::with_capacity(number);
    for i in 0..number {
        keydata_cores.push(KeyDataCore {
            email_address: format!("email_address_{}", i),
            phone_number: None,
            address: None,
        });
    }
    keydata_cores
}

pub fn get_obfuscated_data_cores(number: usize, same_website: bool) -> Vec<ObfuscatedDataCore> {
    let rand_base = 100000;
    let mut rng = rand::thread_rng();

    let mut obfuscated_data_cores = Vec::with_capacity(number);

    for i in 0..number {
        let website = match same_website {
            true => "some_website".to_string(),
            false => format!("some_website_{}", i),
        };

        obfuscated_data_cores.push(ObfuscatedDataCore {
            website_url: website,
            website_id: None,
            obfuscated_email_address: format!(
                "obfuscated_email_address_{}",
                i * rand_base + rng.gen_range(0..rand_base)
            ),
            obfuscated_phone_number: None,
            password: None,
        });
    }
    obfuscated_data_cores
}
