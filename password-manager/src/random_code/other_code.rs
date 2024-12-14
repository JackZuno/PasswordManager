use std::io::{self, Write};
use ring::aead;
use ring::rand::{SystemRandom, SecureRandom};
use ring::pbkdf2;
use rpassword;
use serde::{Serialize, Deserialize};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono;
use std::num::NonZeroU32;

const PBKDF2_ITERATIONS: Option<NonZeroU32> = NonZeroU32::new(100_000);

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

struct PasswordManager {
    master_key: [u8; 32],
    local_passwords: Vec<(String, String, String)>, // Service name, encrypted password, nonce
}

impl PasswordManager {
    fn new(master_password: &str) -> Result<Self, &'static str> {
        // Derive a master key using PBKDF2
        let salt = b"unique-salt-value"; // Replace with a securely generated salt
        let mut master_key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            PBKDF2_ITERATIONS.unwrap(),
            salt,
            master_password.as_bytes(),
            &mut master_key,
        );

        Ok(Self {
            master_key,
            local_passwords: Vec::new(),
        })
    }

    fn add_password(&mut self, service: &str, password: &str) -> Result<(), &'static str> {
        let mut encryption_key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            PBKDF2_ITERATIONS.unwrap(),
            service.as_bytes(),
            &self.master_key,
            &mut encryption_key,
        );

        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &encryption_key)
            .map_err(|_| "Failed to create encryption key")?;
        let key = aead::LessSafeKey::new(unbound_key);

        let nonce = generate_unique_nonce();
        let aad = aead::Aad::empty();
        let mut buffer = password.as_bytes().to_vec();
        buffer.resize(buffer.len() + aead::AES_256_GCM.tag_len(), 0);

        key.seal_in_place_append_tag(aead::Nonce::assume_unique_for_key(nonce), aad, &mut buffer)
            .map_err(|_| "Encryption failed")?;

        let encrypted_password = BASE64_STANDARD.encode(&buffer);
        let encoded_nonce = BASE64_STANDARD.encode(&nonce);

        // Save to local list along with the nonce
        self.local_passwords.push((service.to_string(), encrypted_password.clone(), encoded_nonce.clone()));

        println!("Dio: {:?}, {:?}, {:?}.", service, encrypted_password, encoded_nonce);

        Ok(())
    }

    fn get_password(&self, service: &str) -> Result<String, &'static str> {
        // Find the password entry in the local list
        if let Some((_, encrypted_password, encoded_nonce)) = self.local_passwords.iter().find(|(s, _, _)| s == service) {
            let mut decryption_key = [0u8; 32];
            pbkdf2::derive(
                pbkdf2::PBKDF2_HMAC_SHA256,
                PBKDF2_ITERATIONS.unwrap(),
                service.as_bytes(),
                &self.master_key,
                &mut decryption_key,
            );

            let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &decryption_key)
                .map_err(|_| "Failed to create decryption key")?;
            let key = aead::LessSafeKey::new(unbound_key);

            let nonce = BASE64_STANDARD.decode(encoded_nonce).map_err(|_| "Invalid nonce data")?;

            let buffer = BASE64_STANDARD.decode(encrypted_password).map_err(|_| "Invalid password data")?;

            let aad = aead::Aad::empty();
            let mut buffer = buffer;
            let decrypted = key
                .open_in_place(aead::Nonce::try_assume_unique_for_key(&nonce).map_err(|_| "Invalid nonce")?, aad, &mut buffer)
                .map_err(|_| "Decryption failed")?;

            return Ok(String::from_utf8_lossy(decrypted).to_string());
        }

        Err("Service not found")
    }

    fn authenticate_user(token: &str, secret: &str) -> Result<String, &'static str> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        ).map_err(|_| "Invalid token")?;

        Ok(decoded.claims.sub)
    }

    fn generate_token(user_id: &str, secret: &str) -> String {
        let expiration = chrono::Utc::now().timestamp() as usize + 3600; // 1 hour
        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    }
}

fn generate_unique_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    SystemRandom::new()
        .fill(&mut nonce)
        .expect("Failed to generate nonce");
    nonce
}

fn main() -> Result<(), &'static str> {
    let master_password = rpassword::prompt_password("Enter master password: ")
        .map_err(|_| "Failed to read master password")?;

    let mut manager = PasswordManager::new(&master_password)?;

    println!("Options: 1) Add Password 2) Get Password 3) Exit");
    loop {
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
