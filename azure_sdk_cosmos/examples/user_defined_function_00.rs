use azure_sdk_cosmos::prelude::*;
use futures::stream::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let database = std::env::args()
        .nth(1)
        .expect("please specify database name as first command line parameter");
    let collection = std::env::args()
        .nth(2)
        .expect("please specify collection name as second command line parameter");

    let account = std::env::var("COSMOS_ACCOUNT").expect("Set env variable COSMOS_ACCOUNT first!");
    let master_key =
        std::env::var("COSMOS_MASTER_KEY").expect("Set env variable COSMOS_MASTER_KEY first!");

    let authorization_token = AuthorizationToken::new_master(&master_key)?;

    let client = ClientBuilder::new(account, authorization_token)?;
    let database_client = client.with_database(&database);
    let collection_client = database_client.with_collection(&collection);
    let user_defined_function_client = collection_client.with_user_defined_function(&"test9");

    let ret = user_defined_function_client
        .create_user_defined_function()
        .with_body("body")
        .execute()
        .await?;
    println!("Creeate response object:\n{:#?}", ret);

    let stream = collection_client
        .list_user_defined_functions()
        .with_max_item_count(3)
        .with_consistency_level((&ret).into());
    let mut stream = Box::pin(stream.stream());
    while let Some(ret) = stream.next().await {
        let ret = ret.unwrap();
        println!(
            "List loop received {} items. Object:\n{:#?}",
            ret.item_count, ret
        );
    }

    let ret = user_defined_function_client
        .replace_user_defined_function()
        .with_consistency_level((&ret).into())
        .with_body("new body")
        .execute()
        .await?;
    println!("Replace response object:\n{:#?}", ret);

    let ret = user_defined_function_client
        .delete_user_defined_function()
        .with_consistency_level((&ret).into())
        .execute()
        .await?;

    println!("Delete response object:\n{:#?}", ret);

    Ok(())
}
