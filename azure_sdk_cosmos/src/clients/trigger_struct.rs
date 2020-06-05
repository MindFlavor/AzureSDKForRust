use crate::requests;
use crate::traits::*;
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    collection_client: COLL,
    trigger_name: String,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<C, D, COLL> TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub(crate) fn new(collection_client: COLL, trigger_name: String) -> Self {
        Self {
            collection_client,
            trigger_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<C, D, COLL> HasHyperClient for TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client.hyper_client()
    }
}

impl<C, D, COLL> HasCosmosClient<C> for TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.collection_client.cosmos_client()
    }
}

impl<C, D, COLL> HasDatabaseClient<C, D> for TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.collection_client.database_client()
    }
}

impl<C, D, COLL> HasCollectionClient<C, D, COLL> for TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn collection_client(&self) -> &COLL {
        &self.collection_client
    }
}

impl<C, D, COLL> TriggerClient<C, D, COLL> for TriggerStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    fn create_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    fn replace_trigger(
        &self,
    ) -> requests::CreateOrReplaceTriggerBuilder<'_, C, D, COLL, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    //fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, CUB> {
    //    requests::DeleteTriggerBuilder::new(self)
    //}
}
