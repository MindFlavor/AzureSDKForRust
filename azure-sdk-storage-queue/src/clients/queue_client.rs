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

impl<'a, C> HasStorageClient for QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    type Client = C;

    fn client<'b>(&'b self) -> &'b C {
        self.client.as_ref()
    }
}

impl<'a, C> WithQueueServiceClient<'a> for C
where
    C: Client + Debug + Send + Sync + Clone + 'a,
{
    type Client = C;
    type QueueServiceClient = QueueClient<'a, C>;

    fn with_queue_service_client(&'a self) -> Self::QueueServiceClient {
        QueueClient {
            client: Cow::Borrowed(self),
        }
    }
}

impl<C> IntoQueueServiceClient for C
where
    C: Client + Debug + Send + Sync + Clone + 'static,
{
    type Client = C;
    type QueueServiceClient = QueueClient<'static, C>;

    fn into_queue_service_client(self) -> Self::QueueServiceClient {
        QueueClient {
            client: Cow::Owned(self),
        }
    }
}

impl<'a, C> QueueService for QueueClient<'a, C>
where
    C: Client + Clone + Debug,
{
    fn list_queues(&self) -> requests::ListQueuesBuilder<'_, '_, Self::Client> {
        crate::requests::ListQueuesBuilder::new(self)
    }
}
