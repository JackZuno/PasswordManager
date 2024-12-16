use std::io::{self, Write};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use firestore::FirestoreDb;

use crate::{database::items::{insert_item, retrieve_item_by_account_and_user_with_id, ItemNoId}, menu::menu_functions::generate_password::{calculate_entropy, evaluate_password_strength}, password_functions::password_manager::{derive_master_key, encrypt_password, generate_salt, generate_unique_nonce}};


// ############### ADD NEW ENC PASSWORD ###############
pub async fn add_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let master_password = rpassword::prompt_password("\nEnter master password: ")
        .map_err(|_| "Failed to read master password\n")?;

    if master_password.len() < 12 || master_password.len() > 128 {
        println!("Error: Invalid length for the Master Password (min 12 and max 128 characters).\n");
        return Ok(());
    }

    // Ask the user for the website name
    print!("Enter the website name (minimum 3 characters): ");
    io::stdout().flush().unwrap(); 

    let mut website = String::new();
    io::stdin().read_line(&mut website).map_err(|_| "Invalid input")?;
    let website = website.trim();

    // Validate website name length
    if website.len() < 3 || website.len() > 24 {
        println!("Error: Website name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, website, user).await? {
        Some(_item) => {
            // Account already exists
            println!("Item with name '{}' and user '{}' already exists\n", website, user);

            Ok(())
        }
        None => {
            // Account doesn't exist, you can insert a new password
            print!("Enter the username (minimum 3 characters): ");
            io::stdout().flush().unwrap(); 

            let mut username = String::new();
            io::stdin().read_line(&mut username).map_err(|_| "Invalid input")?;
            let username = username.trim();

            // Validate username length
            if username.len() < 3 || username.len() > 24 {
                println!("Error: Username must be at least 3 characters long and max 24.\n");
                return Ok(());
            }

            print!("Enter new password: ");
            io::stdout().flush().unwrap();

            let mut password = String::new();
            io::stdin().read_line(&mut password).map_err(|_| "Invalid input\n")?;
            let password = password.trim().replace(" ", "");

            if password.len() < 8 || password.len() > 128 {
                println!("Error: Invalid length for the website Password (min 8 and max 128 characters).\n");
                return Ok(());
            }

            let entropy = calculate_entropy(&password);
            println!("Password entropy: {:.4} bits", entropy);

            evaluate_password_strength(entropy);

            // Generate the nonce, the salt and combine it with the master password
            let nonce = generate_unique_nonce();
            let salt = generate_salt();
            let master_key = derive_master_key(&master_password, &salt)?;

            let encryption_result = encrypt_password(&password, &master_key, &nonce, &salt);

            match encryption_result {
                Ok(encrypted_password) => {
                    let now = Local::now();
                    let creation_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

                    let encoded_salt = BASE64_STANDARD.encode(salt);    // [u8; 32]
                    let encoded_nonce = BASE64_STANDARD.encode(nonce);  // [u8; 16]

                    let new_account = ItemNoId {
                        website: website.to_string(),
                        username: username.to_string(),
                        password: encrypted_password,
                        user: user.to_string(),
                        creation_date: creation_date.clone(),
                        last_modified_date: creation_date,
                        salt: encoded_salt,
                        nonce: encoded_nonce,
                    };

                    insert_item(&db, &new_account).await?;

                    Ok(()) 
                },
                Err(err) => {
                    // An error occurred during encryption
                    println!("Error: {}.\n", err);
                    Ok(())
                }
            }
        }
    }
}
