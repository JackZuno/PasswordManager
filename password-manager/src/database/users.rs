use firestore::{struct_path::path, FirestoreDb, FirestoreResult};
use futures::{stream::BoxStream, TryStreamExt};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub username: String,
}


pub async fn add_new_user(
    db: &FirestoreDb,
    user: &User,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "users";

    let email = &user.email;

    let query: BoxStream<FirestoreResult<User> > = db
        .fluent()
        .select()
        .from(collection_name)
        .filter( | q| { 
            q.for_all([
                q.field(path!(User::email)).eq(email),
            ])
        })
        .obj() 
        .stream_query_with_errors()
        .await?;

    let as_vec: Vec<User> = query.try_collect().await?;

    // Check if we got a result
    if !as_vec.is_empty() {
        // The user is already in the database
        Ok(())
    } else {
        // The user is not in the database, adding it now
        add_user_function(&db, &user).await?;

        Ok(()) 
    }
}

async fn add_user_function(
    db: &FirestoreDb,
    user: &User,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Proceed with inserting the item if it does not exist
    let collection_name = "users";

    match db
        .fluent()
        .insert()
        .into(collection_name)
        .generate_document_id()
        .object(user)
        .execute::<()>()
        .await
    {
        Ok(_) => {
            println!("User inserted successfully!\n");

            Ok(())
        }
        Err(err) => {
            // Handle any errors that may occur
            Err(Box::new(err)) 
        }
    }
}
