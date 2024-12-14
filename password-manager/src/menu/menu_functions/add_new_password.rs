use std::io::{self, Write};
use base64::{prelude::BASE64_STANDARD, Engine};
use chrono::Local;
use firestore::FirestoreDb;

use crate::{database::items::{insert_item, retrieve_item_by_account_and_user_with_id, ItemNoId}, password_functions::password_manager::{derive_master_key, encrypt_password, generate_salt, generate_unique_nonce}};


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

    // Ask the user for the account name
    print!("Enter the account name (minimum 3 characters): ");
    io::stdout().flush().unwrap(); 

    let mut account_name = String::new();
    io::stdin().read_line(&mut account_name).map_err(|_| "Invalid input")?;
    let account_name = account_name.trim();

    // Validate account name length
    if account_name.len() < 3 || account_name.len() > 24 {
        println!("Error: Account name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, account_name, user).await? {
        Some(_item) => {
            // Account already exists
            println!("Item with name '{}' and user '{}' already exists\n", account_name, user);

            Ok(())
        }
        None => {
            // Account doesn't exist, you can insert a new password
            print!("Enter new password: ");
            io::stdout().flush().unwrap();

            let mut password = String::new();
            io::stdin().read_line(&mut password).map_err(|_| "Invalid input\n")?;
            let password = password.trim();

            if password.len() < 6 || password.len() > 128 {
                println!("Error: Invalid length for the Account Password (min 6 and max 128 characters).\n");
                return Ok(());
            }

            // Generate the nonce, the salt and combine it with the master password
            let nonce = generate_unique_nonce();
            let salt = generate_salt();
            let master_key = derive_master_key(&master_password, &salt)?;

            let encryption_result = encrypt_password(password, &master_key, &nonce, &salt);

            match encryption_result {
                Ok(encrypted_password) => {
                    let now = Local::now();
                    let creation_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

                    let encoded_salt = BASE64_STANDARD.encode(salt);    // [u8; 32]
                    let encoded_nonce = BASE64_STANDARD.encode(nonce);  // [u8; 16]

                    let new_account = ItemNoId {
                        account_name: account_name.to_string(),
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
