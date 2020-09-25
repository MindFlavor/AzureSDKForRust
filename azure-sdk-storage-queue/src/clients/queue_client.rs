use crate::requests;
use crate::{HasStorageClient, IntoQueueServiceClient, QueueService, WithQueueServiceClient};
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

    pub fn new_owned(client: C) -> Self {
        QueueClient {
            client: Cow::Owned(client),
        }
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

impl<'a, C> WithQueueServiceClient<'a, C, QueueClient<'a, C>> for C
where
    C: Client + Debug + Send + Sync + Clone,
{
    fn with_queue_service_client(&'a self) -> QueueClient<'a, Self> {
        QueueClient {
            client: Cow::Borrowed(self),
        }
    }
}

impl<C> IntoQueueServiceClient<C, QueueClient<'_, C>> for C
where
    C: Client + Debug + Send + Sync + Clone,
{
    fn into_queue_service_client(self) -> QueueClient<'static, Self> {
        QueueClient {
            client: Cow::Owned(self),
        }
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
