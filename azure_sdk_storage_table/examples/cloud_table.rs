#[macro_use]
extern crate serde_derive;

use azure_sdk_storage_table::{CloudTable, Continuation, TableClient};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct MyEntity {
    data: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = TableClient::new(&account, &master_key)?;
    let cloud_table = CloudTable::new(client, "test");
    cloud_table.create_if_not_exists().await?;

    let entity = cloud_table.get::<MyEntity>("pk", "rk").await?;
    println!("entity: {:?}", entity);

    let cnt = 20usize;

    for r in 0usize..cnt {
        let pk = "big2";
        let rk = &format!("{}", r);
        let entity = cloud_table
            .insert(
                pk,
                rk,
                MyEntity {
                    data: "hello".to_owned(),
                },
            )
            .await?;
        if r % 100 == 0 {
            println!("insert {:?}", entity);
        }
    }

    let entity = cloud_table.get::<serde_json::Value>("big2", "0").await?;
    println!("entity(value): {:?}", entity);

    let mut cont = Continuation::start();
    while let Some(entities) = cloud_table
        .execute_query::<MyEntity>(None, &mut cont)
        .await?
    {
        println!("segment: {:?}", entities.first());
    }

    let mut cont = Continuation::start();
    while let Some(entities) = cloud_table
        .execute_query::<serde_json::Value>(None, &mut cont)
        .await?
    {
        println!("segment(value): {:?}", entities.first());
    }

    for r in 0usize..cnt {
        let pk = "big2";
        let rk = &format!("{}", r);
        if r % 100 == 0 {
            println!("delete {}:{}", pk, rk);
        }
        let _ = cloud_table.delete(pk, rk, None).await;
    }

    Ok(())
}
