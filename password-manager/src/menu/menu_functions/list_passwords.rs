use firestore::FirestoreDb;
use prettytable::{Cell, Row, Table, Attr, color};

use crate::database::items::{get_items, ItemList};


pub async fn list_items_menu(
    db: &FirestoreDb,
    user: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let all_items: Vec<ItemList> = get_items(&db, user).await?;

    let mut table = Table::new();
    
    table.add_row(Row::new(vec![
        Cell::new("Website")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::RED)),
        Cell::new("Username")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("Creation Date")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::YELLOW)),
        Cell::new("Last Modified Date")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::MAGENTA)),
    ]));

    for (index, item) in all_items.iter().enumerate() {
        if index %2 == 0 {
            table.add_row(Row::new(vec![
                Cell::new(&item.website)
                    .with_style(Attr::BackgroundColor(color::BLACK))
                    .with_style(Attr::ForegroundColor(color::WHITE)),
                Cell::new(&item.username)
                    .with_style(Attr::BackgroundColor(color::BLACK))
                    .with_style(Attr::ForegroundColor(color::WHITE)),
                Cell::new(&item.creation_date)
                    .with_style(Attr::BackgroundColor(color::BLACK))
                    .with_style(Attr::ForegroundColor(color::WHITE)),
                Cell::new(&item.last_modified_date)
                    .with_style(Attr::BackgroundColor(color::BLACK))
                    .with_style(Attr::ForegroundColor(color::WHITE)),
            ]));
        } else {
            table.add_row(Row::new(vec![
                Cell::new(&item.website)
                    .with_style(Attr::BackgroundColor(color::WHITE))
                    .with_style(Attr::ForegroundColor(color::BLACK)),
                Cell::new(&item.username)
                    .with_style(Attr::BackgroundColor(color::WHITE))
                    .with_style(Attr::ForegroundColor(color::BLACK)),
                Cell::new(&item.creation_date)
                    .with_style(Attr::BackgroundColor(color::WHITE))
                    .with_style(Attr::ForegroundColor(color::BLACK)),
                Cell::new(&item.last_modified_date)
                    .with_style(Attr::BackgroundColor(color::WHITE))
                    .with_style(Attr::ForegroundColor(color::BLACK)),
            ]));
        }
            
    }

    print!("\n");
    table.printstd();
    print!("\n");

    Ok(())
}
