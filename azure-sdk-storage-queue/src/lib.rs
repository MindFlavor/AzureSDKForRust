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

pub trait QueueService<C>: Debug + Send + Sync
where
    C: Client,
{
    fn list_queues<'a, 'b>(&'a self) -> requests::ListQueuesBuilder<'a, 'b, C>;
}
