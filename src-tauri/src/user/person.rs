use rsa::{RsaPrivateKey, RsaPublicKey};
pub struct Person {
    pub address: String,
    pub rng: rand::prelude::ThreadRng,
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
}

impl Person {
    pub fn new(address: String) -> Self {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate a key");
        let pub_key: RsaPublicKey = RsaPublicKey::from(&priv_key);
        Self {
            address,
            rng,

            priv_key,
            pub_key,
        }
    }
}
