extern crate azure_sdk_for_rust;
extern crate chrono;
extern crate env_logger;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate log;
extern crate md5;
extern crate tokio_core;

use azure_sdk_for_rust::prelude::*;
use azure_sdk_for_rust::storage::blob::BlockListType;
use azure_sdk_for_rust::storage::blob::{BlobBlockType, BlockList};
use futures::future::*;
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

    let container = std::env::args()
        .nth(1)
        .expect("please specify container name as command line parameter");
    let blob_name = std::env::args().nth(2).expect("please specify blob name as command line parameter");

    let mut core = Core::new()?;

    let client = Client::new(&account, &master_key)?;

    let data = b"something";

    // this is not mandatory but it helps preventing
    // spurious data to be uploaded.
    let digest = md5::compute(&data[..]);

    // The required parameters are container_name, blob_name and body.
    // The builder supports many more optional
    // parameters (such as LeaseID, or ContentDisposition, MD5 etc...)
    // so make sure to check with the documentation.
    let future = client
        .put_block_blob()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_content_type("text/plain")
        .with_body(&data[..])
        .with_content_md5(&digest[..])
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    let mut block_list = BlockList::default();
    block_list.blocks.push(BlobBlockType::Uncommitted(b"satanasso" as &[u8]));
    block_list.blocks.push(BlobBlockType::Uncommitted(b"pollastro" as &[u8]));

    let future = client
        .put_block()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_body(&data[..])
        .with_block_id(b"satanasso" as &[u8])
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    let future = client
        .put_block()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_body(&data[..])
        .with_block_id(b"pollastro" as &[u8])
        .finalize();
    core.run(future.map(|res| println!("{:?}", res)))?;

    let future = client
        .get_block_list()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_block_list_type(BlockListType::All)
        .finalize();

    let ret = core.run(future)?;
    println!("GetBlockList == {:?}", ret);

    let bl = ret.block_with_size_list.into();
    println!("bl == {:?}", bl);

    let future = client
        .put_block_list()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_block_list(&bl)
        .finalize();
    core.run(future.map(|res| println!("PutBlockList == {:?}", res)))?;

    let future = client
        .acquire_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_duration(60)
        .finalize();
    let res = core.run(future)?;
    println!("Acquire lease == {:?}", res);

    let future = client
        .renew_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_id(&res.lease_id)
        .finalize();
    let res = core.run(future)?;
    println!("Renew lease == {:?}", res);

    let future = client
        .release_blob_lease()
        .with_container_name(&container)
        .with_blob_name(&blob_name)
        .with_lease_id(&res.lease_id)
        .finalize();
    let res = core.run(future)?;
    println!("Release lease == {:?}", res);

    Ok(())
}
