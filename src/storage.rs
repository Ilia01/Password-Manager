use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
};

use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead}; // AES-GCM

use rand::{TryRngCore, rngs::OsRng};

pub fn display_passwords(key_bytes: &[u8; 32]) -> io::Result<()> {
    let path = Path::new("./passwords.txt");
    let file = File::open(path)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 4 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let service = parts[0];
        let username = parts[1];

        let nonce_bytes = match base64::decode(parts[2]) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Invalid nonce for {}|{}", service, username);
                continue;
            }
        };
        let ciphertext = match base64::decode(parts[3]) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("Invalid ciphertext for {}|{}", service, username);
                continue;
            }
        };

        let nonce = Nonce::from_slice(&nonce_bytes);

        let decrypted = match cipher.decrypt(nonce, ciphertext.as_ref()) {
            Ok(p) => String::from_utf8_lossy(&p).to_string(),
            Err(_) => {
                eprintln!("Failed to decrypt password for {}|{}", service, username);
                continue;
            }
        };

        println!(
            "Service: {}\nUsername: {}\nPassword: {}\n",
            service, username, decrypted
        );
    }

    Ok(())
}

pub fn add_new_password(
    service: &str,
    username: &str,
    password: &str,
    key_bytes: &[u8; 32],
) -> io::Result<()> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(&key);

    let mut nonce_bytes = [0u8; 12];
    let _ = OsRng.try_fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, password.as_bytes())
        .expect("Encryption failed");

    let line = format!(
        "{}|{}|{}|{}\n",
        service,
        username,
        base64::encode(nonce_bytes),
        base64::encode(ciphertext)
    );

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("./passwords.txt")?;
    file.write_all(line.as_bytes())?;
    Ok(())
}
