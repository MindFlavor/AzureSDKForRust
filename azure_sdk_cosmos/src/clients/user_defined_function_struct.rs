use crate::requests;
use crate::traits::*;
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct UserDefinedFunctionStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    collection_client: COLL,
    user_defined_function_name: String,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<C, D, COLL> UserDefinedFunctionStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub(crate) fn new(collection_client: COLL, user_defined_function_name: String) -> Self {
        Self {
            collection_client,
            user_defined_function_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<C, D, COLL> HasHyperClient for UserDefinedFunctionStruct<C, D, COLL>
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

impl<C, D, COLL> HasCosmosClient<C> for UserDefinedFunctionStruct<C, D, COLL>
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

impl<C, D, COLL> HasDatabaseClient<C, D> for UserDefinedFunctionStruct<C, D, COLL>
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

impl<C, D, COLL> HasCollectionClient<C, D, COLL> for UserDefinedFunctionStruct<C, D, COLL>
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

impl<C, D, COLL> UserDefinedFunctionClient<C, D, COLL> for UserDefinedFunctionStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn user_defined_function_name(&self) -> &str {
        &self.user_defined_function_name
    }

    fn create_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, true)
    }

    fn replace_user_defined_function(
        &self,
    ) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, '_, C, D, COLL, No> {
        requests::CreateOrReplaceUserDefinedFunctionBuilder::new(self, false)
    }

    //fn delete_user_defined_function(&self) -> requests::DeleteUserDefinedFunctionBuilder<'_, CUB> {
    //    requests::DeleteUserDefinedFunctionBuilder::new(self)
    //}
}
