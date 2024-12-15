use std::io::{self, Write};

use colored::*;
use firestore::FirestoreDb;
use tokio::{sync::mpsc, task};

use crate::{database::users::{add_new_user, User}, menu::menu_layout::menu_function, user_auth::{help::help_function, login_functions_folder::{listener::start_listener, login_function::login_with_google}}};


pub async fn menu_authentication(
    db: &FirestoreDb,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        println!("{}", "----------------------------------------".bold().bright_purple());
        println!(
            "{}\n\t{} {}\n\t{} {}\n\t{} {} {}\n",
            "Options:".bold().underline(),
            "[1]".green(), "Log in with Google".bold(),
            "[2]".yellow(), "Help".bold(),
            "[3]".red(), "Exit".bold(), "(Close the program)".dimmed()
        );

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                // Create a channel to receive the authorization code
                let (tx, rx) = mpsc::channel(1);

                // Start the listener in a new background task
                task::spawn(async {
                    // This should start the Axum listener that waits for the OAuth2 redirect
                    start_listener(tx).await;
                });
                
                match login_with_google(rx).await {
                    Ok(Some(user_info)) => {
                        println!("\nUser Details:");
                        println!("Name: {}", user_info.name);
                        println!("Email: {}", user_info.email);

                        let user = User {
                            username: user_info.name,
                            email: user_info.email
                        };

                        add_new_user(&db, &user).await?;

                        menu_function(db, &user).await?
                    }
                    Ok(None) => println!("Login failed or was canceled."),
                    Err(e) => println!("Error during login: {}", e),
                }
            }
            "2" => {
                help_function();
            }
            "3" => {
                println!("{}\n", "Closing the program. Goodbye!".bright_red().bold());
                break;
            }
            _ => println!("{}\n", "Invalid choice. Please try again.".bright_red()),
        }
    }

    Ok(())
}