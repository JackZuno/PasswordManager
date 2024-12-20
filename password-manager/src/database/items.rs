use firestore::*;
use futures::{stream::BoxStream, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use gcloud_sdk::google::firestore::v1::value::ValueType;


// ############### STRUCTURES ###############
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemNoId {
    pub website: String,
    pub username: String,
    pub password: String,
    pub user: String,
    pub creation_date: String,
    pub last_modified_date: String,
    pub salt: String,
    pub nonce: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemList {
    pub website: String,
    pub username: String,
    pub password: String,
    pub user: String,
    pub creation_date: String,
    pub last_modified_date: String,
    pub document_id: String,
    pub salt: String,
    pub nonce: String,
}


// ############### INSERT ITEM ###############
pub async fn insert_item(
    db: &FirestoreDb,
    item: &ItemNoId,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match retrieve_item_by_account_and_user(db, &item.website, &item.user).await? {
        Some(_existing_item) => {
            println!("Item with website name '{}' and user '{}' already exists", item.website, item.user);
            return Ok(()); 
        },
        None => {
            // Proceed with inserting the item if it does not exist
            let collection_name = "items";

            match db
                .fluent()
                .insert()
                .into(collection_name)
                .generate_document_id()
                .object(item)
                .execute::<()>()
                .await
            {
                Ok(_) => {
                    println!("Website inserted successfully!\n");

                    Ok(())
                }
                Err(err) => {
                    // Handle any errors that may occur
                    Err(Box::new(err)) 
                }
            }
        }
    }
}


// ############### RETRIEVE ITEM BY ACCOUNT NAME AND USER ###############
pub async fn retrieve_item_by_account_and_user(
    db: &FirestoreDb,
    website: &str,
    user: &str,
) -> Result<Option<ItemNoId>, Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "items";

    let query: BoxStream<FirestoreResult<ItemNoId> > = db
        .fluent()
        .select()
        .from(collection_name)
        .filter( | q| { // Fluent filter API example
            q.for_all([
                q.field(path!(ItemNoId::website)).eq(website),
                q.field(path!(ItemNoId::user)).eq(user),       
            ])
        })
        .obj() // Reading documents as structures using Serde gRPC deserializer
        .stream_query_with_errors()
        .await?;

    let as_vec: Vec<ItemNoId> = query.try_collect().await?;

    if !as_vec.is_empty() {
        Ok(Some(as_vec[0].clone())) 
    } else {
        Ok(None) 
    }
}


// ############### RETRIEVE ITEM BY ACCOUNT NAME AND USER WITH DOC ID ###############
pub async fn retrieve_item_by_account_and_user_with_id(
    db: &FirestoreDb,
    website: &str,
    user: &str,
) -> Result<Option<ItemList>, Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "items";

    let mut stream = db
        .fluent()
        .select()
        .from(collection_name)
        .filter(|q| {
            q.for_all([
                q.field(path!(ItemList::website)).eq(website),
                q.field(path!(ItemList::user)).eq(user),
            ])
        })
        .stream_query_with_metadata()
        .await?;

    let mut items = Vec::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(metadata_with_doc) => {
                let document = metadata_with_doc.document; 

                if let Some(document) = document {
                    let document_id = document
                        .name
                        .rsplit('/')
                        .next()
                        .unwrap_or_default()
                        .to_string();

                    // Extract fields from the document
                    let fields = document.fields;

                    let website = fields
                        .get("website")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let username = fields
                        .get("username")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let password = fields
                        .get("password")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let user = fields
                        .get("user")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let creation_date = fields
                        .get("creation_date")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let last_modified_date = fields
                        .get("last_modified_date")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let salt = fields
                        .get("salt")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let nonce = fields
                        .get("nonce")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    // Create the ItemList object with document_id
                    let item = ItemList {
                        website,
                        username,
                        password,
                        user,
                        creation_date,
                        last_modified_date,
                        document_id,
                        salt,
                        nonce,
                    };

                    items.push(item);
                }
            }
            Err(err) => {
                eprintln!("Error retrieving document: {err}");
            }
        }
    }

    // Return the first item found (if any)
    if let Some(item) = items.first().cloned() {
        Ok(Some(item))
    } else {
        Ok(None)
    }
}




// ############### GET ALL ITEMS ###############
pub async fn get_items(
    db: &FirestoreDb,
    user: &str,
) -> Result<Vec<ItemList>, Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "items";

    // Use fluent interface to query Firestore with metadata
    let mut stream = db
        .fluent()
        .select()
        .from(collection_name)
        .filter( | q| { 
            q.for_all([
                q.field(path!(ItemList::user)).eq(user),
            ])
        })
        .stream_query_with_metadata()
        .await?;

    // Prepare a vector for results
    let mut items = Vec::new();

    while let Some(result) = stream.next().await {
        match result {
            Ok(metadata) => {
                // Unwrap document and extract document_id from the document name
                if let Some(document) = metadata.document {
                    let document_id = document
                        .name
                        .rsplit('/')
                        .next()
                        .unwrap_or_default()
                        .to_string();

                    // Extract fields from the document
                    let fields = document.fields;

                    // Safely extract field values using pattern matching
                    let website = fields
                        .get("website")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let username = fields
                        .get("username")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let password = fields
                        .get("password")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let user = fields
                        .get("user")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let creation_date = fields
                        .get("creation_date")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let last_modified_date = fields
                        .get("last_modified_date")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let salt = fields
                        .get("salt")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    let nonce = fields
                        .get("nonce")
                        .and_then(|v| match &v.value_type {
                            Some(ValueType::StringValue(s)) => Some(s.clone()),
                            _ => None,
                        })
                        .unwrap_or_default();

                    // Create the ItemList object
                    let item = ItemList {
                        website,
                        username,
                        password,
                        user,
                        creation_date,
                        last_modified_date,
                        document_id,
                        salt, 
                        nonce,
                    };

                    items.push(item);
                }
            }
            Err(err) => {
                eprintln!("Error retrieving document: {err}");
            }
        }
    }

    Ok(items)
}



// ############### UPDATE ACCOUNT PASSWORD ###############
pub async fn update_password_db(
    db: &FirestoreDb,
    item: &ItemList,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "items";

    match db
        .fluent()
        .update()
        .fields(paths!(ItemNoId::{password, last_modified_date, salt, nonce}))
        .in_col(&collection_name)
        .document_id(item.document_id.clone())
        .object(item)
        .execute::<()>()
        .await
    {
        Ok(_) => {
            println!("Password updated successfully for the website: {}.\n", item.website);
            Ok(())
        }
        Err(err) => {
            eprintln!("Error updating password: {}", err);
            Err(Box::new(err))
        }
    }
}


// ############### DELETE DOCUMENT BY ID ###############
pub async fn delete_item_by_id(
    db: &FirestoreDb,
    document_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let collection_name = "items";

    // Delete documents
    db.fluent()
        .delete()
        .from(collection_name)
        .document_id(document_id)
        .execute()
        .await?;

    Ok(())
}
