use firestore::FirestoreDb;
use std::io::{self, Write};
use colored::*;

use crate::database::users::User;
use crate::menu::menu_functions::list_passwords::list_items_menu;
use crate::menu::menu_functions::remove_password::remove_password_function;
use crate::menu::menu_functions::retrieve_password::retrieve_password_function;
use crate::menu::menu_functions::update_password::update_password_function;
use crate::menu::menu_functions::add_new_password::add_password_function;


pub async fn menu_function(
    db: &FirestoreDb,
    user: &User,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let email = user.email.clone();
    let username = user.username.clone();

    print!("\n");
    loop {
        println!("{}", "----------------------------------------".bold().bright_purple());
        println!(
            "{}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n\t{} {} {}\n",
            "Options:".bold().underline(),
            "[1]".green(), "List saved passwords".bold(), "(Displays all accounts and credentials)".dimmed(),
            "[2]".green(), "Add a new password".bold(), "(Store a new account and password)".dimmed(),
            "[3]".green(), "Retrieve a password".bold(), "(Get the password for an account)".dimmed(),
            "[4]".green(), "Update a password".bold(), "(Modify an existing password)".dimmed(),
            "[5]".green(), "Remove a password".bold(), "(Delete an account and password)".dimmed(),
            "[6]".red(), "Logout".bold(), "(Go Back to the starting menu)".dimmed()
        );   
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).map_err(|_| "Invalid input")?;
    
        match choice.trim() {
            "1" => { // list accounts
                list_items_menu(&db, &email).await?;
            }
            "2" => { //add new account and password
                add_password_function(&db, &email).await?;
            }
            "3" => { // retrieve a password
                retrieve_password_function(&db, &email).await?;
            }
            "4" => { // Update a password
                update_password_function(&db, &email).await?;
            }
            "5" => { // Remove a password                
                remove_password_function(&db, &email).await?;
            }
            "6" => {
                println!("{} {}{}\n", "Loggin out. Goodbye".bright_red().bold(), username.bright_red().italic(), "!".bright_red().bold());
                break
            },
            _ => println!("{}\n", "Invalid choice. Please try again.".bright_red()),
        }
    }

    Ok(())
}
