use azure_sdk_core::errors::AzureError;
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_table::{CloudTable, Continuation2, TableClient, TableEntity};
use env_logger;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct MyEntity {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        //.filter_level(log::LevelFilter::Trace)
        .filter_module("azure_sdk_storage_table", log::LevelFilter::Trace);

    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = TableClient::new(&account, &master_key)?;
    let cloud_table = CloudTable::new(client, "test");
    cloud_table.create_if_not_exists().await?;

    let entity = cloud_table.get_entity::<MyEntity>("pk", "rk").await?;
    println!("entity: {:?}", entity);

    for r in 0..2000 {
        let pk = "big2";
        let rk = &format!("{}", r);
        println!("delete {}:{}", pk, rk);
        let _ = cloud_table.delete_entity(pk, rk, None).await;
    }

    let mut cont = Continuation2::start();
    while let Some(entities) = cloud_table
        .query_entities::<MyEntity>(None, &mut cont)
        .await?
    {
        println!("segment: {:?}", entities.first());
    }

    Ok(())
}
