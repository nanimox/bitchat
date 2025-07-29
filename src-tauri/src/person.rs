use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
pub struct Person {
    address: String,
    rng: rand::prelude::ThreadRng,
    bits: usize,
    priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

impl Person {
    pub fn new(address: String) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let bits = 2048;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key: RsaPublicKey = RsaPublicKey::from(&priv_key);
        Self {
            address,
            rng,
            bits,
            priv_key,
            pub_key,
        }
    }
    pub fn encrypt(&mut self, data: String, pub_key: &RsaPublicKey) -> String {
        // Encrypt
        let encrypted = pub_key
            .encrypt(&mut self.rng, Pkcs1v15Encrypt, &data.as_bytes())
            .expect("failed to encrypt");
        Self::my_encrypt(&encrypted)
    }

    pub fn decrypt(&self, enc_data: &String) -> Vec<u8> {
        // Decrypt
        self.priv_key
            .decrypt(Pkcs1v15Encrypt, &Self::my_decrypt(enc_data))
            .expect("failed to decrypt")
    }

    fn my_encrypt(data: &Vec<u8>) -> String {
        println!("{data:?}");
        let mut done: String = String::new();
        for item in data {
            done.push_str(&format!("{item}x"));
        }
        println!("{done}");
        done
    }

    fn my_decrypt(data: &String) -> Vec<u8> {
        let sla = data.split('x');
        let mut full_data: Vec<u8> = vec![];
        for item in sla {
            full_data.push(item.parse().unwrap_or(0));
        }
        full_data.pop();
        full_data
    }
}
