// use firestore::FirestoreDb;
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub username: String,
}


// pub async fn add_new_user(
//     db: &FirestoreDb,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     //

//     Ok(())
// }
