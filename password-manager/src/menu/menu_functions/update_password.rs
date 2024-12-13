use std::io::{self, Write};
use chrono::Local;
use firestore::FirestoreDb;

use crate::database::items::{retrieve_item_by_account_and_user_with_id, update_password_db};

pub async fn update_password_function(
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
        Some(mut item) => {
            // Account exists, prompt for new password
            print!("Enter new password: ");
            io::stdout().flush().unwrap();

            let mut new_password = String::new();
            io::stdin().read_line(&mut new_password).map_err(|_| "Invalid input")?;
            let new_password = new_password.trim();

            // Update the password and last modified date
            let now = Local::now();
            let last_modified_date = now.format("%Y-%m-%d %H:%M:%S").to_string();

            item.password = new_password.to_string();
            item.last_modified_date = last_modified_date;

            // Update the Firestore database
            update_password_db(&db, &item).await?;

            Ok(())
        }
        None => {
            // Account doesn't exist, print a message and return
            println!("\nNo account found with name: {}\n", account_name);
            Ok(()) // Return success without updating
        }
    }
}

