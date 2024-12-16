use firestore::FirestoreDb;
use std::io::{self, Write};

use crate::database::items::{delete_item_by_id, retrieve_item_by_account_and_user_with_id};

pub async fn remove_password_function(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Ask the user for the account name
    print!("Enter the website name: ");
    io::stdout().flush().unwrap(); 

    let mut website = String::new();
    io::stdin().read_line(&mut website).map_err(|_| "Invalid input\n")?;
    let website = website.trim(); 

    // Validate website name length
    if website.len() < 3 || website.len() > 24 {
        println!("Error: website name must be at least 3 characters long and max 24.\n");
        return Ok(());
    }

    // Check if the website exists
    match retrieve_item_by_account_and_user_with_id(db, website, user).await? {
        Some(item) => {
            // website exists, prompt for new password
            println!("Are you sure you want to remove the website '{}'?", website);
            println!("Type 'Y' for Yes or 'N' for No");

            let mut confirmation = String::new();
            io::stdin().read_line(&mut confirmation).map_err(|_| "Invalid input\n")?;
            let confirmation = confirmation.trim().to_uppercase();

            if confirmation == "Y" {
                // Proceed to remove the website
                let document_id = item.document_id;

                delete_item_by_id(&db, &document_id).await?;

                println!("The website '{}' has been removed.\n", website);
            } else if confirmation == "N" {
                println!("Operation cancelled. The account '{}' was not removed.\n", website);
            } else {
                println!("Invalid input. Operation cancelled.\n");
            }

            Ok(())
        }
        None => {
            println!("No account found with name: {}\n", website);
            Ok(()) 
        }
    }
}
