use firestore::FirestoreDb;
use std::io::{self, Write};

use crate::menu::menu_functions::list_passwords::list_items_menu;
use crate::menu::menu_functions::update_password::update_password_function;
use crate::menu::menu_functions::add_new_password::add_password_function;

pub async fn menu_function(
    db: &FirestoreDb,
)  -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        println!("Options: \n\t1) List the saved passwords \n\t2) Add a new password \n\t3) Update a password \n\t4) Remove a password \n\t5) Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
    
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).map_err(|_| "Invalid input")?;
    
        match choice.trim() {
            "1" => {
                let user = "pippo@gmail.com";

                list_items_menu(&db, user).await?;
            }
            "2" => {
                let user = "pippo@gmail.com";

                add_password_function(&db, &user).await?;
            }
            "3" => {
                let user = "pippo@gmail.com";
                update_password_function(&db, user).await?;
            }
            "4" => {
                print!("Enter service name: ");
                io::stdout().flush().unwrap();
    
                let mut service = String::new();
                io::stdin().read_line(&mut service).map_err(|_| "Invalid input")?;
            }
            "5" => break,
            _ => println!("\nInvalid choice\n"),
        }
    }

    Ok(())
}
