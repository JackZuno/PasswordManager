use std::collections::HashMap;
use std::io::{self, Write};
use ring::aead;
use ring::rand::{SystemRandom, SecureRandom};
use ring::digest::{Context, SHA256};
use rpassword;

struct PasswordManager {
    passwords: HashMap<String, Vec<u8>>, // Service -> Encrypted password
    key: aead::LessSafeKey,
    nonce: [u8; 12],
}

impl PasswordManager {
    fn new(master_password: &str) -> Result<Self, &'static str> {
        // Derive a key from the master password
        let key_bytes = derive_key(master_password);

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
            .map_err(|_| "Failed to create encryption key")?;

        let key = aead::LessSafeKey::new(unbound_key);

        // Generate a random nonce
        let mut nonce = [0u8; 12];
        SystemRandom::new()
            .fill(&mut nonce)
            .map_err(|_| "Failed to generate nonce")?;

        Ok(Self {
            passwords: HashMap::new(),
            key,
            nonce,
        })
    }

    fn add_password(&mut self, service: &str, password: &str) -> Result<(), &'static str> {
        let aad = aead::Aad::empty();
        let mut buffer = password.as_bytes().to_vec();
        buffer.resize(buffer.len() + aead::AES_256_GCM.tag_len(), 0);

        self.key
            .seal_in_place_append_tag(aead::Nonce::assume_unique_for_key(self.nonce), aad, &mut buffer)
            .map_err(|_| "Encryption failed")?;

        self.passwords.insert(service.to_string(), buffer);
        Ok(())
    }

    fn get_password(&self, service: &str) -> Result<String, &'static str> {
        let aad = aead::Aad::empty();
        if let Some(encrypted_password) = self.passwords.get(service) {
            let mut buffer = encrypted_password.clone();
            let decrypted = self.key
                .open_in_place(aead::Nonce::assume_unique_for_key(self.nonce), aad, &mut buffer)
                .map_err(|_| "Decryption failed")?;

            return Ok(String::from_utf8_lossy(decrypted).to_string());
        }

        Err("Service not found")
    }
}

fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Context::new(&SHA256);
    hasher.update(password.as_bytes());

    let hash = hasher.finish();
    let mut key = [0u8; 32];
    key.copy_from_slice(&hash.as_ref()[..32]);
    key
}

fn main() -> Result<(), &'static str> {
    let master_password = rpassword::prompt_password("Enter master password: ")
        .map_err(|_| "Failed to read master password")?;

    let mut manager = PasswordManager::new(&master_password)?;

    loop {
        println!("Options: 1) Add Password 2) Get Password 3) Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).map_err(|_| "Invalid input")?;

        match choice.trim() {
            "1" => {
                print!("Enter service name: ");
                io::stdout().flush().unwrap();

                let mut service = String::new();
                io::stdin().read_line(&mut service).map_err(|_| "Invalid input")?;

                print!("Enter password: ");
                io::stdout().flush().unwrap();

                let mut password = String::new();
                io::stdin().read_line(&mut password).map_err(|_| "Invalid input")?;

                manager.add_password(service.trim(), password.trim())?;
                println!("Password added successfully!");
            }
            "2" => {
                print!("Enter service name: ");
                io::stdout().flush().unwrap();

                let mut service = String::new();
                io::stdin().read_line(&mut service).map_err(|_| "Invalid input")?;

                match manager.get_password(service.trim()) {
                    Ok(password) => println!("Password: {}", password),
                    Err(err) => println!("Error: {}", err),
                }
            }
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }

    Ok(())
}
