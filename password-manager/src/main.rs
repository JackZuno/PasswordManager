// use std::io::{self, Write};
// use ring::aead;
// use ring::rand::{SystemRandom, SecureRandom};
// use ring::digest::{Context, SHA256};
// use rpassword;
// use reqwest;
// use serde::{Serialize, Deserialize};
// use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
// use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
// use base64::Engine;
use serde_json::Value;
use firebase_rs::*;


// #[derive(Serialize, Deserialize)]
// struct Claims {
//     sub: String,
//     exp: usize,
// }

// struct PasswordManager {
//     database_url: String,
//     key: aead::LessSafeKey,
//     nonce: [u8; 12],
// }

// impl PasswordManager {
//     fn new(master_password: &str, database_url: &str) -> Result<Self, &'static str> {
//         // Derive a key from the master password
//         let key_bytes = derive_key(master_password);

//         let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, &key_bytes)
//             .map_err(|_| "Failed to create encryption key")?;

//         let key = aead::LessSafeKey::new(unbound_key);

//         // Generate a random nonce
//         let mut nonce = [0u8; 12];
//         SystemRandom::new()
//             .fill(&mut nonce)
//             .map_err(|_| "Failed to generate nonce")?;

//         Ok(Self {
//             database_url: database_url.to_string(),
//             key,
//             nonce,
//         })
//     }

//     fn add_password(&self, service: &str, password: &str) -> Result<(), &'static str> {
//         let aad = aead::Aad::empty();
//         let mut buffer = password.as_bytes().to_vec();
//         buffer.resize(buffer.len() + aead::AES_256_GCM.tag_len(), 0);

//         self.key
//             .seal_in_place_append_tag(aead::Nonce::assume_unique_for_key(self.nonce), aad, &mut buffer)
//             .map_err(|_| "Encryption failed")?;

//         let encrypted_password = BASE64_STANDARD.encode(&buffer);

//         let client = reqwest::blocking::Client::new();
//         let res = client.post(&self.database_url)
//             .json(&serde_json::json!({"service": service, "password": encrypted_password}))
//             .send();

//         if res.is_err() || !res.unwrap().status().is_success() {
//             return Err("Failed to save password to database");
//         }

//         Ok(())
//     }

//     fn get_password(&self, service: &str) -> Result<String, &'static str> {
//         let client = reqwest::blocking::Client::new();
//         let res = client.get(&format!("{}/{}", self.database_url, service)).send();

//         if let Ok(response) = res {
//             if response.status().is_success() {
//                 let data: serde_json::Value = response.json().map_err(|_| "Failed to parse response")?;
//                 let encrypted_password = BASE64_STANDARD.decode(data["password"].as_str().unwrap_or("")).map_err(|_| "Invalid data received")?;

//                 let aad = aead::Aad::empty();
//                 let mut buffer = encrypted_password.clone();
//                 let decrypted = self.key
//                     .open_in_place(aead::Nonce::assume_unique_for_key(self.nonce), aad, &mut buffer)
//                     .map_err(|_| "Decryption failed")?;

//                 return Ok(String::from_utf8_lossy(decrypted).to_string());
//             }
//         }

//         Err("Service not found")
//     }

    // fn authenticate_user(token: &str, secret: &str) -> Result<String, &'static str> {
    //     let decoded = decode::<Claims>(
    //         token,
    //         &DecodingKey::from_secret(secret.as_ref()),
    //         &Validation::default(),
    //     ).map_err(|_| "Invalid token")?;

    //     Ok(decoded.claims.sub)
    // }

    // fn generate_token(user_id: &str, secret: &str) -> String {
    //     let expiration = chrono::Utc::now().timestamp() as usize + 3600; // 1 hour
    //     let claims = Claims {
    //         sub: user_id.to_string(),
    //         exp: expiration,
    //     };

    //     encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
    // }
// }

// fn derive_key(password: &str) -> [u8; 32] {
//     let mut hasher = Context::new(&SHA256);
//     hasher.update(password.as_bytes());

//     let hash = hasher.finish();
//     let mut key = [0u8; 32];
//     key.copy_from_slice(&hash.as_ref()[..32]);
//     key
// }

// fn main() -> Result<(), &'static str> {
//     let master_password = rpassword::prompt_password("Enter master password: ")
//         .map_err(|_| "Failed to read master password")?;

//     let database_url = "https://your-database-url.com/api/passwords";

//     let manager = PasswordManager::new(&master_password, database_url)?;

//     println!("Options: 1) Add Password 2) Get Password 3) Exit");
//     loop {
//         print!("Enter choice: ");
//         io::stdout().flush().unwrap();

//         let mut choice = String::new();
//         io::stdin().read_line(&mut choice).map_err(|_| "Invalid input")?;

//         match choice.trim() {
//             "1" => {
//                 print!("Enter service name: ");
//                 io::stdout().flush().unwrap();

//                 let mut service = String::new();
//                 io::stdin().read_line(&mut service).map_err(|_| "Invalid input")?;

//                 print!("Enter password: ");
//                 io::stdout().flush().unwrap();

//                 let mut password = String::new();
//                 io::stdin().read_line(&mut password).map_err(|_| "Invalid input")?;

//                 manager.add_password(service.trim(), password.trim())?;
//                 println!("Password added successfully!");
//             }
//             "2" => {
//                 print!("Enter service name: ");
//                 io::stdout().flush().unwrap();

//                 let mut service = String::new();
//                 io::stdin().read_line(&mut service).map_err(|_| "Invalid input")?;

//                 match manager.get_password(service.trim()) {
//                     Ok(password) => println!("Password: {}", password),
//                     Err(err) => println!("Error: {}", err),
//                 }
//             }
//             "3" => break,
//             _ => println!("Invalid choice"),
//         }
//     }

//     Ok(())
// }

fn main() {
    print!("DIO CANE\n")

}
