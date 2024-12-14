use firestore::FirestoreDb;
use std::io::{self, Write};
use colored::*;

use crate::menu::menu_functions::list_passwords::list_items_menu;
use crate::menu::menu_functions::remove_password::remove_password_function;
use crate::menu::menu_functions::retrieve_password::retrieve_password_function;
use crate::menu::menu_functions::update_password::update_password_function;
use crate::menu::menu_functions::add_new_password::add_password_function;


pub async fn menu_function(
    db: &FirestoreDb,
)  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    print!("\n");
    loop {
        println!(
            "{}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n",
            "Options:".bold().underline(),
            "[1]".green(), "List saved passwords".bold(), "(Displays all accounts and credentials)".dimmed(),
            "[2]".green(), "Add a new password".bold(), "(Store a new account and password)".dimmed(),
            "[3]".green(), "Retrieve a password".bold(), "(Get the password for an account)".dimmed(),
            "[4]".green(), "Update a password".bold(), "(Modify an existing password)".dimmed(),
            "[5]".green(), "Remove a password".bold(), "(Delete an account and password)".dimmed(),
            "[6]".red(), "Exit".bold(), "(Close the program)".dimmed()
        );   
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).map_err(|_| "Invalid input")?;
    
        match choice.trim() {
            "1" => { // list accounts
                let user = "pippo@gmail.com";

                list_items_menu(&db, user).await?;
            }
            "2" => { //add new account and password
                let user = "pippo@gmail.com";

                add_password_function(&db, &user).await?;
            }
            "3" => { // retrieve a password
                let user = "pippo@gmail.com";

                retrieve_password_function(&db, &user).await?;
            }
            "4" => { // Update a password
                let user = "pippo@gmail.com";

                update_password_function(&db, user).await?;
            }
            "5" => { // Remove a password
                let user = "pippo@gmail.com";
                
                remove_password_function(&db, user).await?;
            }
            "6" => break,
            _ => println!("\nInvalid choice\n"),
        }
    }

    Ok(())
}
