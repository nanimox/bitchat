use super::utils::Utils;
use crate::{db::vault::Vault, user::person::Person};
use rsa::Pkcs1v15Encrypt;

pub struct RSA;

impl RSA {
    pub fn encrypt(owner: &mut Person, data: String, person: &Person) {
        // Encrypt
        let encrypted = person
            .pub_key
            .encrypt(&mut owner.rng, Pkcs1v15Encrypt, &data.as_bytes())
            .expect("failed to encrypt");
        let data = Utils::num_vec_to_x_string(&encrypted);
        Vault::store(person, &data)
    }

    pub fn decrypt(owner: &Person) -> Vec<String> {
        // Get data
        let enc_data = Vault::read(owner);
        // Decrypt
        let mut total: Vec<String> = vec![];
        for line in &enc_data {
            println!("Loop: {line}");
            let my_dec = Utils::x_string_to_num_vec(line);
            println!("{my_dec:?}");
            total.append(&mut vec![Utils::stringfy(RSA::temp(owner, my_dec))]);
            println!("NOT HERE@");
        }
        total
    }
    pub fn temp(owner: &Person, data: Vec<u8>) -> Vec<u8> {
        match owner.priv_key.decrypt(Pkcs1v15Encrypt, &data) {
            Ok(vec) => return vec,
            Err(_) => return vec![],
        };
    }
}
