use std::io::{self, Write};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use firestore::FirestoreDb;

use crate::{database::items::{retrieve_item_by_account_and_user_with_id, update_password_db}, password_functions::password_manager::{derive_master_key, encrypt_password, generate_salt, generate_unique_nonce}};

pub async fn update_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Ask the user for the account name
    print!("Enter the account name: ");
    io::stdout().flush().unwrap(); 

    let mut account_name = String::new();
    io::stdin().read_line(&mut account_name).map_err(|_| "Invalid input")?;
    let account_name = account_name.trim(); 

    // Validate account name length
    if account_name.len() < 3 || account_name.len() > 24 {
        println!("Error: Account name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    let master_password = rpassword::prompt_password("Enter master password: ")
        .map_err(|_| "Failed to read master password\n")?;

    if master_password.len() < 12 || master_password.len() > 128 {
        println!("Error: Invalid length for the Master Password (min 12 and max 128 characters).\n");
        return Ok(());
    }

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, account_name, user).await? {
        Some(mut item) => {
            // Account exists, prompt for new password
            print!("Enter the new password: ");
            io::stdout().flush().unwrap();

            let mut new_password = String::new();
            io::stdin().read_line(&mut new_password).map_err(|_| "Invalid input")?;
            let new_password = new_password.trim();

            if new_password.len() < 6 || new_password.len() > 128 {
                println!("Error: Invalid length for the Account Password (min 6 and max 128 characters).\n");
                return Ok(());
            }

            // Update the password and last modified date
            let now = Local::now();
            let last_modified_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let nonce = generate_unique_nonce();
            let salt = generate_salt();
            let master_key = derive_master_key(&master_password, &salt)?;

            let encryption_result = encrypt_password(new_password, &master_key, &nonce, &salt);

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
            println!("No account found with name: {}\n", account_name);
            Ok(())
        }
    }
}

