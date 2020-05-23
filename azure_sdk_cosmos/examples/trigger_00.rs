use azure_sdk_cosmos::prelude::*;
use azure_sdk_cosmos::trigger::*;
use futures::stream::StreamExt;
use std::error::Error;

const TRIGGER_BODY: &str = r#"
function updateMetadata() {
    var context = getContext();
    var collection = context.getCollection();
    var response = context.getResponse();
    var createdDocument = response.getBody();

    // query for metadata document
    var filterQuery = 'SELECT * FROM root r WHERE r.id = \"_metadata\"';
    var accept = collection.queryDocuments(collection.getSelfLink(), filterQuery,
        updateMetadataCallback);
    if (!accept) throw\ "Unable to update metadata, abort\";

    function updateMetadataCallback(err, documents, responseOptions) {
        if (err) throw new Error(\"Error\" + err.message);
            if (documents.length != 1) throw 'Unable to find metadata document';
            var metadataDocument = documents[0];

            // update metadata
            metadataDocument.createdDocuments += 1; metadataDocument.createdNames += \" \" + createdDocument.id;
            var accept = collection.replaceDocument(metadataDocument._self,
                metadataDocument,
                function(err, docReplaced) {
                    if (err) throw\ "Unable to update metadata, abort\";
                });
            if (!accept) throw\ "Unable to update metadata, abort\";
            return;
        }
}"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");
    let trigger_name = std::env::args()
        .nth(3)
        .expect("please specify trigger name as third command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = ClientBuilder::new(account, authorization_token)?;
    let database_client = client.with_database(&database);
    let collection_client = database_client.with_collection(&collection);
    let trigger_client = collection_client.with_trigger(&trigger_name);

    let ret = trigger_client
        .create_trigger()
        .with_trigger_type(TriggerType::Post)
        .with_trigger_operation(TriggerOperation::All)
        .with_body(&TRIGGER_BODY)
        .execute()
        .await?;
    println!("Creeate response object:\n{:#?}", ret);

    Ok(())
}
