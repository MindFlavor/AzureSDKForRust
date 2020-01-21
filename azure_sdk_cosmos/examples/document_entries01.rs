use azure_sdk_cosmos::prelude::*;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct MySampleStruct<'a> {
    a_string: Cow<'a, str>,
    a_number: u64,
    a_timestamp: i64,
}

// This example expects you to have created a collection
// with partitionKey on "id".
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database_name = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection_name = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");
    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = ClientBuilder::new(account, authorization_token)?;
    let client = client.with_database(&database_name);
    let client = client.with_collection(&collection_name);

    let doc = Document::new(
        format!("unique_id{}", 500),
        MySampleStruct {
            a_string: Cow::Borrowed("Something here"),
            a_number: 600,
            a_timestamp: chrono::Utc::now().timestamp(),
        },
    );

    let mut partition_keys = PartitionKeys::new();
    partition_keys.push(doc.document_attributes.id())?;

    // let's add an entity.
    let create_document_response = client
        .create_document()
        .with_document(&doc)
        .with_partition_keys(&partition_keys)
        .with_is_upsert(true)
        .execute()
        .await?;

    println!(
        "create_document_response == {:#?}\n\n\n",
        create_document_response
    );

    let document_client = client.with_document(&doc);

    let get_document_response = document_client
        .get_document()
        .with_partition_keys(&partition_keys)
        .execute::<serde_json::Value>()
        .await?;
    println!("get_document_response == {:#?}", get_document_response);

    let document_client = client.with_document(&"ciccia");

    let get_document_response = document_client
        .get_document()
        .with_partition_keys(&partition_keys)
        .execute::<serde_json::Value>()
        .await?;
    println!(
        "get_document_response == {:#?}\n\n\n",
        get_document_response
    );

    let list_documents_response = client
        .list_documents()
        .execute::<serde_json::Value>()
        .await?;
    println!("list_documents_response == {:#?}", list_documents_response);

    Ok(())
}
