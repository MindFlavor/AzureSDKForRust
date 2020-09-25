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

pub trait HasStorageClient: Debug + Send + Sync {
    type Client: Client + Debug + Send + Sync;
    fn client(&self) -> &Self::Client;
}

pub trait QueueService: HasStorageClient + Debug + Send + Sync {
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, Self::Client>;
    //fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, C>;
}

pub trait WithQueueServiceClient<'a>: Debug + Send + Sync {
    type Client: Client + Debug + Send + Sync;
    type QueueServiceClient: QueueService;

    fn with_queue_service_client(&'a self) -> Self::QueueServiceClient;
}

pub trait IntoQueueServiceClient: Debug + Send + Sync {
    type Client: Client + Debug + Send + Sync;
    type QueueServiceClient: QueueService;

    fn into_queue_service_client(self) -> Self::QueueServiceClient;
}

//*************
