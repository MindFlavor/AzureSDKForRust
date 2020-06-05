use crate::requests;
use crate::traits::*;
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct StoredProcedureStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    collection_client: COLL,
    stored_procedure_name: String,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<C, D, COLL> StoredProcedureStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub(crate) fn new(collection_client: COLL, stored_procedure_name: String) -> Self {
        Self {
            collection_client,
            stored_procedure_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<C, D, COLL> HasHyperClient for StoredProcedureStruct<C, D, COLL>
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

impl<C, D, COLL> HasCosmosClient<C> for StoredProcedureStruct<C, D, COLL>
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

impl<C, D, COLL> HasDatabaseClient<C, D> for StoredProcedureStruct<C, D, COLL>
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

impl<C, D, COLL> HasCollectionClient<C, D, COLL> for StoredProcedureStruct<C, D, COLL>
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

impl<C, D, COLL> StoredProcedureClient<C, D, COLL> for StoredProcedureStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn stored_procedure_name(&self) -> &str {
        &self.stored_procedure_name
    }

    fn create_stored_procedure(
        &self,
    ) -> requests::CreateStoredProcedureBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateStoredProcedureBuilder::new(self)
    }

    //fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, CUB, No> {
    //    requests::ReplaceStoredProcedureBuilder::new(self)
    //}

    fn execute_stored_procedure(
        &self,
    ) -> requests::ExecuteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::ExecuteStoredProcedureBuilder::new(self)
    }

    fn delete_stored_procedure(
        &self,
    ) -> requests::DeleteStoredProcedureBuilder<'_, '_, C, D, COLL> {
        requests::DeleteStoredProcedureBuilder::new(self)
    }
}
