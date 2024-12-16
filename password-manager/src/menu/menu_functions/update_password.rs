use std::io::{self, Write};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use firestore::FirestoreDb;

use crate::{database::items::{retrieve_item_by_account_and_user_with_id, update_password_db}, menu::menu_functions::generate_password::{calculate_entropy, evaluate_password_strength}, password_functions::password_manager::{derive_master_key, encrypt_password, generate_salt, generate_unique_nonce}};

pub async fn update_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Ask the user for the website name
    print!("Enter the website name: ");
    io::stdout().flush().unwrap(); 

    let mut website = String::new();
    io::stdin().read_line(&mut website).map_err(|_| "Invalid input")?;
    let website = website.trim(); 

    // Validate website name length
    if website.len() < 3 || website.len() > 24 {
        println!("Error: website name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    let master_password = rpassword::prompt_password("Enter master password: ")
        .map_err(|_| "Failed to read master password\n")?;

    if master_password.len() < 12 || master_password.len() > 128 {
        println!("Error: Invalid length for the Master Password (min 12 and max 128 characters).\n");
        return Ok(());
    }

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, website, user).await? {
        Some(mut item) => {
            // Account exists, prompt for new password
            print!("Enter the new password: ");
            io::stdout().flush().unwrap();

            let mut new_password = String::new();
            io::stdin().read_line(&mut new_password).map_err(|_| "Invalid input")?;
            let new_password = new_password.trim().replace(" ", "");

            if new_password.len() < 8 || new_password.len() > 128 {
                println!("Error: Invalid length for the website Password (min 8 and max 128 characters).\n");
                return Ok(());
            }

            let entropy = calculate_entropy(&new_password);
            println!("Password entropy: {:.4} bits", entropy);

            evaluate_password_strength(entropy);

            // Update the password and last modified date
            let now = Local::now();
            let last_modified_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let nonce = generate_unique_nonce();
            let salt = generate_salt();
            let master_key = derive_master_key(&master_password, &salt)?;

            let encryption_result = encrypt_password(&new_password, &master_key, &nonce, &salt);

            match encryption_result {
                Ok(encrypted_password) => {

                    let encoded_salt = BASE64_STANDARD.encode(salt);    // [u8; 32]
                    let encoded_nonce = BASE64_STANDARD.encode(nonce);  // [u8; 16]

                    item.last_modified_date = last_modified_date;
                    item.salt = encoded_salt;
                    item.nonce = encoded_nonce;
                    item.password = encrypted_password;

                    update_password_db(&db, &item).await?;

                    Ok(()) 
                },
                Err(err) => {
                    println!("Error: {}.\n", err);
                    Ok(())
                }
            }
        }
        None => {
            // Account doesn't exist, print a message and return
            println!("No website found with name: {}\n", website);
            Ok(())
        }
    }
}

