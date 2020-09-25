#![warn(unused_extern_crates)]
#![recursion_limit = "128"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate azure_sdk_core;

pub mod prelude;
pub mod requests;
pub mod responses;
use azure_sdk_storage_core::Client;
use core::fmt::Debug;
mod clients;
pub use clients::*;

pub trait HasStorageClient<C>: Debug + Send + Sync
where
    C: Client + Debug + Send + Sync,
{
    fn client(&self) -> &C;
}

pub trait QueueService<C>: HasStorageClient<C> + Debug + Send + Sync
where
    C: Client + Debug + Send + Sync,
{
    //    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, C>;
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, C, Self>
    where
        Self: Sized;
}

//*************
