use firestore::FirestoreDb;
use std::io::{self, Write};

use crate::database::items::{delete_item_by_id, retrieve_item_by_account_and_user_with_id};

pub async fn remove_password_function(
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
        Some(item) => {
            // Account exists, prompt for new password
            println!("\nAre you sure you want to remove the account '{}'?", account_name);
            println!("Type 'Y' for Yes or 'N' for No");

            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation).map_err(|_| "Invalid input")?;
            let confirmation = confirmation.trim().to_uppercase();

            if confirmation == "Y" {
                // Proceed to remove the account
                let document_id = item.document_id;

                delete_item_by_id(&db, &document_id).await?;

                println!("\nThe account '{}' has been removed.\n", account_name);
            } else if confirmation == "N" {
                println!("\nOperation cancelled. The account '{}' was not removed.\n", account_name);
            } else {
                println!("\nInvalid input. Operation cancelled.\n");
            }

            

            Ok(())
        }
        None => {
            // Account doesn't exist, print a message and return
            println!("\nNo account found with name: {}\n", account_name);
            Ok(()) // Return success without updating
        }
    }
}
