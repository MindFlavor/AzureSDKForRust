#[macro_use]
extern crate serde_derive;
use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::{TableService, TableStorage};
use azure_sdk_storage_table::TableEntity;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct MyEntity {
    pub my_value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let table_name = std::env::args()
        .nth(1)
        .expect("pass the table name as first command line parameter.");

    let row_key = std::env::args()
        .nth(2)
        .expect("pass the row key as second command line parameter.");

    let client = Client::new(&account, &master_key)?;
    let table_service = TableService::new(client);
    let table_storage = TableStorage::new(table_service, table_name);
    table_storage.create_if_not_exists().await?;

    let my_entity = TableEntity {
        row_key: row_key,
        partition_key: "100".to_owned(),
        etag: None,
        payload: {
            MyEntity {
                my_value: "Itsy bitsy spider".to_owned(),
            }
        },
    };

    // insert the entity
    let mut my_entity = table_storage.insert_entity(my_entity).await?;
    println!("entity inserted: {:?}", my_entity);

    // get the entity (notice the etag)
    let ret: TableEntity<MyEntity> = table_storage
        .get_entity(&my_entity.partition_key, &my_entity.row_key)
        .await?
        .ok_or(AzureError::GenericErrorWithText(
            "item not found after insertion".to_string(),
        ))?;
    println!("get_entity result == {:?}", ret);

    // now we update the entity passing the etag.
    my_entity.payload.my_value = "Wheel on the bus".to_owned();

    let my_entity = table_storage.update_entity(my_entity).await?;
    println!("update_entity completed without errors: {:?}", my_entity);

    // get the entity again (new payload and etag)
    let ret: TableEntity<MyEntity> = table_storage
        .get_entity(&my_entity.partition_key, &my_entity.row_key)
        .await?
        .ok_or(AzureError::GenericErrorWithText(
            "item not found after update".to_string(),
        ))?;
    println!("get_entity result == {:?}", ret);

    Ok(())
}
