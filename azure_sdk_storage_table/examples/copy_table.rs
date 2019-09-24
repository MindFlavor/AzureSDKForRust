use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::table::TableService;
use futures::future::*;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account names and master keys from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");
    let to_account = std::env::var("TO_STORAGE_ACCOUNT").expect("Set env variable TO_STORAGE_ACCOUNT first!");
    let to_master_key = std::env::var("TO_STORAGE_MASTER_KEY").expect("Set env variable TO_STORAGE_MASTER_KEY first!");

    let table_name = std::env::args()
        .nth(1)
        .expect("please specify table name as command line parameter");

    let mut core = Core::new()?;

    let table_service = TableService::new(Client::new(&account, &master_key)?);
    let to_table_service = TableService::new(Client::new(&to_account, &to_master_key)?);

    println!("creating table {}", &table_name);
    let create_table = to_table_service.create_table(&table_name);
    core.run(create_table)?;

    let copy_entities = table_service.query_entities(&table_name, None).and_then(move |entities: Vec<serde_json::Value>| {
        println!("copying {} entities to table {}", entities.len(), &table_name);
        let inserts = entities.into_iter().map(move |entity| {
            to_table_service.insert_entity(&table_name, &entity)
        });
        join_all(inserts)
    });
    core.run(copy_entities)?;

    Ok(())
}