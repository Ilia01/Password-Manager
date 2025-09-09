use std::fs;

use argon2::Argon2;
use rand::rngs::OsRng;
use rand::{Rng, TryRngCore, rng};

pub fn generate_password() -> String {
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                           abcdefghijklmnopqrstuvwxyz\
                           0123456789\
                           !@#$%^&*()_-+=[{]}\\;:'\",<.>/?";
    let mut rng = rng();
    let password = (0..16)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    password
}

pub fn load_or_generate_salt() -> [u8; 16] {
    let path = "./salt.bin";
    if let Ok(data) = fs::read(path) {
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&data);
        salt
    } else {
        let mut salt = [0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt);
        fs::write(path, &salt).expect("Failed to write salt file");
        salt
    }
}

pub fn derive_key(master_password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let argon = Argon2::default();
    let mut key = [0u8; 32];

    argon
        .hash_password_into(master_password.as_bytes(), salt, &mut key)
        .expect("Key derivation failed");
    key
}
