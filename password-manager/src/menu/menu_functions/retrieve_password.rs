use base64::Engine;
use firestore::FirestoreDb;
use std::io::{self, Write};
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;

use crate::{database::items::retrieve_item_by_account_and_user_with_id, password_functions::password_manager::{decrypt_password, derive_master_key}};

pub async fn retrieve_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Ask the user for the account name
    print!("Enter the account name: ");
    io::stdout().flush().unwrap(); 

    let mut account_name = String::new();
    io::stdin().read_line(&mut account_name).map_err(|_| "Invalid input\n")?;
    let account_name = account_name.trim(); 

    if account_name.len() < 3 || account_name.len() > 24 {
        println!("Error: Account name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, account_name, user).await? {
        Some(item) => {
            let master_password = rpassword::prompt_password("Enter master password: ")
                .map_err(|_| "Failed to read master password\n")?;

            if master_password.len() < 12 || master_password.len() > 128 {
                println!("Error: Invalid length for the Master Password (min 12 and max 128 characters).\n");
                return Ok(());
            }

            let salt_vec = BASE64_STANDARD.decode(&item.salt).unwrap();
            let salt: [u8; 32] = salt_vec.try_into().map_err(|_| "Invalid salt length\n")?;

            let nonce = item.nonce;

            let master_key = derive_master_key(&master_password, &salt)?;

            match decrypt_password(&master_key, &nonce, &item.password, &salt) {
                Ok(decrypted_password) => {
                    println!("The password for the account {:?} is: {:?}.\n", item.account_name, decrypted_password);
                }
                Err(err) => {
                    println!("Failed to decrypt the password: {}.\n", err);
                }
            }

            Ok(())
        }
        None => {
            println!("No account found with name: {}\n", account_name);
            Ok(())
        }
    }
}
