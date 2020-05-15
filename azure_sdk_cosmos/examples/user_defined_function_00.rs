use azure_sdk_cosmos::prelude::*;
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

    let ret = client
        .with_database(&database)
        .with_collection(&collection)
        .with_user_defined_function(&"test2")
        .create_user_defined_function()
        .with_body("body")
        .execute()
        .await?;

    println!("Response object:\n{:#?}", ret);

    let ret = client
        .with_database(&database)
        .with_collection(&collection)
        .with_user_defined_function(&"test2")
        .delete_user_defined_function()
        .with_consistency_level((&ret).into())
        .execute()
        .await?;

    println!("Response object:\n{:#?}", ret);

    Ok(())
}
