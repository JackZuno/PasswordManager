use firestore::FirestoreDb;
use prettytable::{Cell, Row, Table};

use crate::database::items::{get_items, ItemList};

pub async fn list_items_menu(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let all_items: Vec<ItemList> = get_items(&db, user).await?;

    let mut table = Table::new();
    
    table.add_row(Row::new(vec![
        Cell::new("Account Name"),
        Cell::new("User"),
        Cell::new("Creation Date"),
        Cell::new("Last Modified Date"),
    ]));

    for item in &all_items {
        table.add_row(Row::new(vec![
            Cell::new(&item.account_name),
            Cell::new(&item.user),
            Cell::new(&item.creation_date),
            Cell::new(&item.last_modified_date),
        ]));
    }

    print!("\n");
    table.printstd();
    print!("\n");

    Ok(())
}
