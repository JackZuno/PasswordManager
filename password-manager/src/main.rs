use chrono::Local;
use database::items::{get_items, insert_item, ItemList, ItemNoId};
use firestore::*;
use dotenv::dotenv;
use menu::menu_layout::menu_function;
use std::env;

mod menu {
    mod menu_functions {
        pub mod list_passwords;
        pub mod update_password;
        pub mod add_new_password;
    }
    pub mod menu_layout;
}

mod database {
    pub mod items;
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

    // // Authenticate user
    // let authenticator = InstalledFlowAuthenticator::builder(
    //     google_auth_oauth2::read_application_secret(json_path).await?,
    // )
    // .build()
    // .await?;

    // // INSERT SOME DATA INTO THE DATABASE

    // // Insert1
    // let now = Local::now();
    // let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // println!("\nAdd new item 1");
    // let new_item_1 = ItemNoId {
    //     account_name: "Twitter".to_string(),
    //     password: "kjfngjhgdvtetv34tvw3434c".to_string(),
    //     user: "pippo@gmail.com".to_string(),
    //     creation_date: formatted_time.clone(),
    //     last_modified_date: formatted_time,
    // };

    // insert_item(&db, &new_item_1).await?;

    // // Insert2
    // let now_2 = Local::now();
    // let formatted_time_2 = now_2.format("%Y-%m-%d %H:%M:%S").to_string();

    // println!("\nAdd new item 2");

    // let new_item_2 = ItemNoId {
    //     account_name: "YouTube".to_string(),
    //     password: "fnjhfhvyty45654364356f435".to_string(),
    //     user: "pippo@gmail.com".to_string(),
    //     creation_date: formatted_time_2.clone(),
    //     last_modified_date: formatted_time_2,
    // };

    // insert_item(&db, &new_item_2).await?;

    // // Insert3
    // let now_3 = Local::now();
    // let formatted_time_3 = now_3.format("%Y-%m-%d %H:%M:%S").to_string();

    // println!("\nAdd new item 3");

    // let new_item_3 = ItemNoId {
    //     account_name: "YouTube".to_string(),
    //     password: "sggyhydtvrthchdfghc__-fds".to_string(),
    //     user: "test@gmailil.com".to_string(),
    //     creation_date: formatted_time_3.clone(),
    //     last_modified_date: formatted_time_3,
    // };

    // insert_item(&db, &new_item_3).await?;


    // // RETRIEVE ITEMS

    // println!("\nRetrieve Items");
    // let all_items: Vec<ItemList> = get_items(&db).await?;

    // for item in all_items {
    //     println!("{:?}", item);
    // }

    let _menu = menu_function(&db).await?;

    Ok(())
}



//   // Update documents
//   let object_updated: MyTestStructure = db.fluent()
//       .update()
//       .fields(paths!(MyTestStructure::{age})) // Update only specified fields
//       .in_col(TEST_COLLECTION_NAME)
//       .document_id(&my_struct.some_id)
//       .object(&MyTestStructure {
//           age: my_struct.age + 1,
//           ..my_struct.clone()
//       })
//       .execute()
//      .await?;
 
//   // Get a document as an object by id
//   let find_it_again: Option<MyTestStructure> = db.fluent()
//         .select()
//         .by_id_in(TEST_COLLECTION_NAME)
//         .obj()
//         .one(&my_struct.some_id)
//         .await?;

//   // Query and read stream of objects
//   let object_stream: BoxStream<MyTestStructure> = db.fluent()
//     .select()
//     .fields(paths!(MyTestStructure::{some_id, age, num})) // Optionally select the fields needed
//     .from(TEST_COLLECTION_NAME)
//     .filter(|q| { // Fluent filter API example
//         q.for_all([
//             q.field(path!(MyTestStructure::age)).is_not_null(),
//             q.field(path!(MyTestStructure::num)).eq("Test"),
//             // Sometimes you have optional filters
//             Some("Test2")
//                 .and_then(|value| q.field(path!(MyTestStructure::one_more_string)).eq(value)),
//         ])
//     })
//     .order_by([(
//         path!(MyTestStructure::some_num),
//         FirestoreQueryDirection::Descending,
//     )])
//     .obj() // Reading documents as structures using Serde gRPC deserializer
//     .stream_query()
//     .await?;

    // let as_vec: Vec<MyTestStructure> = object_stream.collect().await;
    // println!("{:?}", as_vec);

    // // Delete documents
    // db.fluent()
    //     .delete()
    //     .from(TEST_COLLECTION_NAME)
    //     .document_id(&my_struct.some_id)
    //     .execute()
    //     .await?;
