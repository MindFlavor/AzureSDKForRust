extern crate azure_sdk_for_rust;

extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate tokio_core;

use azure_sdk_for_rust::prelude::*;
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    env_logger::init();
    code().unwrap();
}

// We run a separate method to use the elegant quotation mark operator.
// A series of unwrap(), unwrap() would have achieved the same result.
fn code() -> Result<(), Box<Error>> {
    // First we retrieve the account name and master key from environment variables.
    let account = std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let master_key = std::env::var("STORAGE_MASTER_KEY").expect("Set env variable STORAGE_MASTER_KEY first!");

    let container_name = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");

    let mut core = Core::new()?;
    let client = Client::new(Account::Azure { account, key: master_key })?;

    let future = client
        .list_blobs()
        .with_container_name(&container_name)
        .with_include_copy()
        .with_include_deleted()
        .with_include_metadata()
        .with_include_snapshots()
        .with_include_uncommitted_blobs()
        .finalize();

    let _res = core.run(future)?;

    let future = client
        .get_blob()
        .with_container_name(&container_name)
        .with_blob_name("SorgeniaReorganizeRebuildIndexes.zip")
        .finalize();
    let result = core.run(future)?;

    println!("{:?}", result);

    Ok(())
}
