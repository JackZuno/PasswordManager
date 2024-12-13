use std::io::{self, Write};
use chrono::Local;
use firestore::FirestoreDb;

use crate::database::items::{insert_item, retrieve_item_by_account_and_user_with_id, ItemNoId};

pub async fn add_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Ask the user for the account name
    print!("\nEnter the account name: ");
    io::stdout().flush().unwrap(); 

    let mut account_name = String::new();
    io::stdin().read_line(&mut account_name).map_err(|_| "Invalid input")?;
    let account_name = account_name.trim();

    // Check if the account exists
    match retrieve_item_by_account_and_user_with_id(db, account_name, user).await? {
        Some(_item) => {
            // Account already exists
            println!("\nItem with account_name '{}' and user '{}' already exists\n", account_name, user);

            Ok(())
        }
        None => {
            // Account doesn't exist, you can insert a new password
            print!("Enter new password: ");
            io::stdout().flush().unwrap();

            let mut password = String::new();
            io::stdin().read_line(&mut password).map_err(|_| "Invalid input")?;
            let password = password.trim();

            // Update the password and last modified date
            let now = Local::now();
            let creation_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let new_account = ItemNoId {
                account_name: account_name.to_string(),
                password: password.to_string(),
                user: user.to_string(),
                creation_date: creation_date.clone(),
                last_modified_date: creation_date,
            };

            // Add the password into the Firestore database
            insert_item(&db, &new_account).await?;

            Ok(()) // Return success without updating
        }
    }
}
