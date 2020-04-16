use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::responses::GetDocumentResponse;
use futures::stream::StreamExt;
use std::borrow::Cow;
use std::error::Error;
#[macro_use]
extern crate serde_derive;

// Now we create a sample struct. The Cow trick
// allows us to use the same struct for serializing
// (without having to own the items if not needed) and
// for deserializing (where owning is required).
// We do not need to define the "id" field here, it will be
// specified in the Document struct below.
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

    let id = format!("unique_id{}", 100);

    let doc = Document::new(
        id.clone(),
        MySampleStruct {
            a_string: Cow::Borrowed("Something here"),
            a_number: 100,
            a_timestamp: chrono::Utc::now().timestamp(),
        },
    );

    // let's add an entity.
    match client
        .create_document()
        .with_document(&doc)
        .with_partition_keys(PartitionKeys::new().push(doc.document_attributes.id())?)
        .execute()
        .await
    {
        Ok(_) => {
            println!("document created");
        }
        Err(err) => {
            println!("already exists? ==> {:?}", err);
        }
    };

    let mut partition_keys = PartitionKeys::new();
    partition_keys.push(doc.document_attributes.id())?;
    let document_client = client.with_document(&id, &partition_keys);

    let ret = document_client.list_attachments().execute().await?;

    println!("{:#?}", ret);

    let attachment_client = document_client.with_attachment(&"myattach5");

    let resp = attachment_client
        .create_slug()
        .with_content_type("text/plain")
        .with_body(b"FFFFF")
        .execute()
        .await?;

    println!("resp == {:#?}", resp);

    Ok(())
}
