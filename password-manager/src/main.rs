use firestore::*;
use dotenv::dotenv;
use menu::menu_layout::menu_function;
use std::env;


mod menu {
    mod menu_functions {
        pub mod list_passwords;
        pub mod update_password;
        pub mod add_new_password;
        pub mod remove_password;
        pub mod retrieve_password;
    }
    pub mod menu_layout;
}

mod database {
    pub mod items;
    pub mod users;
}

mod password_functions {
    pub mod password_manager;
}


fn get_env_variable(variable_name: &str) -> String {
    return match env::var(variable_name) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: {:?} environment variable is not set.", variable_name);
            std::process::exit(1);
        }
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok(); // Reads the .env file

    // Set firestore database
    let project_id = get_env_variable("PROJECT_ID");
    let json_path = get_env_variable("JSON_PATH");

    let env_json_path = "GOOGLE_APPLICATION_CREDENTIALS";
    std::env::set_var(env_json_path, json_path);
    
    // Create an instance
    let db = FirestoreDb::new(&project_id).await?;

    let _menu = menu_function(&db).await?;

    Ok(())
}
