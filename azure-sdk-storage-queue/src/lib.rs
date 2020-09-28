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
use azure_sdk_core::No;
use azure_sdk_storage_core::Client;
use core::fmt::Debug;
mod clients;
pub use clients::*;
use std::borrow::Cow;

//********* Request traits
pub trait VisibilityTimeoutSupport {
    type O;
    fn with_visibility_timeout_seconds(self, timeout: u64) -> Self::O;
}

pub trait VisibilityTimeoutRequired {
    fn visibility_timeout_seconds(&self) -> u64;

    fn to_uri_parameter(&self) -> String {
        format!("visibilitytimeout={}", self.visibility_timeout_seconds())
    }
}

pub trait MessageTTLSupport {
    type O;
    fn with_message_ttl_seconds(self, timeout: u64) -> Self::O;
}

pub trait MessageTTLRequired {
    fn message_ttl_seconds(&self) -> u64;

    fn to_uri_parameter(&self) -> String {
        format!("messagettl={}", self.message_ttl_seconds())
    }
}

pub trait MessageBodySupport<'b> {
    type O;
    fn with_message_body<BODY: Into<Cow<'b, str>>>(self, body: BODY) -> Self::O;
}

pub trait MessageBodyRequired {
    fn message_body<'b>(&self) -> &str;
}

//********* Queue service traits
pub trait HasStorageClient: Debug + Send + Sync {
    type StorageClient: Client;
    fn storage_client(&self) -> &Self::StorageClient;
}

pub trait QueueService: HasStorageClient + Sync {
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, Self::StorageClient>;
}

pub trait WithQueueServiceClient<'a>: Debug + Send + Sync + Clone {
    type QueueServiceClient: QueueService;

    fn with_queue_service_client(&'a self) -> Self::QueueServiceClient;
}

pub trait IntoQueueServiceClient: Debug + Send + Sync + Clone {
    type QueueServiceClient: QueueService;

    fn into_queue_service_client(self) -> Self::QueueServiceClient;
}

//*************
pub trait QueueNameService: HasStorageClient {
    fn queue_name(&self) -> &str;

    fn put_message(&self) -> requests::PutMessageBuilder<'_, '_, Self::StorageClient, No>;
}

pub trait WithQueueNameClient<'a, 'b>: Debug + Send + Sync + Clone {
    type QueueNameClient: QueueNameService;

    fn with_queue_name_client<NAME>(&'a self, queue_name: NAME) -> Self::QueueNameClient
    where
        NAME: Into<Cow<'b, str>>;
}

pub trait IntoQueueNameClient<'b>: Debug + Send + Sync + Clone {
    type QueueNameClient: QueueNameService;

    fn into_queue_name_client<NAME>(self, queue_name: NAME) -> Self::QueueNameClient
    where
        NAME: Into<Cow<'b, str>>;
}
