use azure_sdk_cosmos::prelude::*;
use serde::{Deserialize, Serialize};
use warp::{reject, Filter};

#[derive(Serialize, Deserialize, Debug)]
struct MySampleStruct {
    id: String,
    name: String,
}

#[tokio::main]
async fn main() {
    let mincase = warp::path("mincase")
        .and(warp::path::end())
        .and(warp::get())
        .and_then(mincase);
    warp::serve(mincase).run(([127, 0, 0, 1], 3030)).await
}

pub async fn mincase() -> Result<impl warp::Reply, warp::Rejection> {
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let database =
        std::env::var("COSMOS_DATABASE").expect("Set env variable COSMOS_DATABASE first!");
    let collection_name =
        std::env::var("COSMOS_COLLECTION").expect("Set env variable COSMOS_COLLECTION first!");

    let u = MySampleStruct {
        id: String::from("test"),
        name: String::from("test"),
    };

    // Create cosmos partition key.
    let mut pk = PartitionKeys::new();
    let pk = match pk.push(&u.name) {
        Ok(pk) => pk,
        Err(_) => {
            return Err(reject::reject());
        }
    };

    // Prepare document for inserting.
    let document_to_insert = Document::new(&u);

    let authorization_token = match AuthorizationToken::new_master(&master_key) {
        Ok(t) => t,
        Err(_) => {
            return Err(reject::reject());
        }
    };

    let client = match ClientBuilder::new(account, authorization_token) {
        Ok(c) => c,
        Err(_) => {
            return Err(reject::reject());
        }
    };

    let database_client = client.with_database_client(database);
    let collection_client = database_client.with_collection_client(collection_name);
    let c = collection_client
        .create_document()
        .with_partition_keys(pk)
        .with_is_upsert(true);

    let f = c.execute_with_document(&document_to_insert).await;

    match f {
        Ok(_) => {
            println!("ALPHA");
        }
        Err(error) => {
            println!("BRAVO {}.", error);
        }
    }

    Ok(warp::reply::json(&u))
}
