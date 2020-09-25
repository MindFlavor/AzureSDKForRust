#[macro_use]
extern crate log;

use azure_sdk_core::prelude::*;
use azure_sdk_storage_core::prelude::*;
use azure_sdk_storage_queue::prelude::*;
use std::borrow::Cow;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key =
        std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let client = client::with_access_key(&account, &master_key);

    let client = azure_sdk_storage_queue::QueueClient::new(Cow::Owned(client));

    trace!("enumerating queues");

    let response = client.list_queues().execute().await?;

    println!("response == {:?}", response);

    Ok(())
}
