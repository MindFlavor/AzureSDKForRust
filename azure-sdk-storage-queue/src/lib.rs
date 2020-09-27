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
    type StorageClient: Client;
    fn storage_client(&self) -> &Self::StorageClient;
}

pub trait QueueService: HasStorageClient {
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, Self::StorageClient>;
}

pub trait WithQueueServiceClient<'a>: Debug + Send + Sync + Clone {
    type StorageClient: Client;
    type QueueServiceClient: QueueService;

    fn with_queue_service_client(&'a self) -> Self::QueueServiceClient;
}

pub trait IntoQueueServiceClient: Debug + Send + Sync + Clone {
    type StorageClient: Client;
    type QueueServiceClient: QueueService;

    fn into_queue_service_client(self) -> Self::QueueServiceClient;
}

//*************
