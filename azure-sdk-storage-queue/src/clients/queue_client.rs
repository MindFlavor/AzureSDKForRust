use crate::requests;
use crate::{HasStorageClient, QueueService};
use azure_sdk_storage_core::Client;
use std::borrow::Cow;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    pub client: Cow<'a, C>,
}

impl<'a, C> QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    pub fn new(client: Cow<'a, C>) -> Self {
        QueueClient { client }
    }
}

impl<'a, C> HasStorageClient<C> for QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    fn client<'b>(&'b self) -> &'b C {
        self.client.as_ref()
    }
}

impl<'a, C> QueueService<C> for QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, C> {
        crate::requests::ListQueuesBuilder::new(self)
    }
}
