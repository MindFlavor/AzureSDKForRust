use crate::clients::*;
use crate::requests;
use crate::{
    CollectionClient, CosmosClient, DatabaseClient, HasCosmosClient, HasDatabaseClient,
    HasHyperClient, IntoDocumentClient, PartitionKeys,
};
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    database_client: D,
    collection_name: String,
    p_c: PhantomData<C>,
}

impl<C, D> CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(database_client: D, collection_name: String) -> Self {
        Self {
            database_client,
            collection_name,
            p_c: PhantomData {},
        }
    }
}

impl<C, D> HasHyperClient for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }
}

impl<C, D> HasCosmosClient<C> for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client.cosmos_client()
    }
}

impl<C, D> HasDatabaseClient<C, D> for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<C, D> CollectionClient<C, D> for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn collection_name(&self) -> &str {
        &self.collection_name
    }

    fn get_collection(&self) -> requests::GetCollectionBuilder<'_, C, D> {
        requests::GetCollectionBuilder::new(self)
    }

    fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, C, D> {
        requests::DeleteCollectionBuilder::new(self)
    }

    fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_, C, D, No, No> {
        requests::ReplaceCollectionBuilder::new(self)
    }

    fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, C, D> {
        requests::ListDocumentsBuilder::new(self)
    }

    fn create_document(&self) -> requests::CreateDocumentBuilder<'_, '_, C, D, No> {
        requests::CreateDocumentBuilder::new(self)
    }

    fn replace_document(&self) -> requests::ReplaceDocumentBuilder<'_, '_, C, D, No, No> {
        requests::ReplaceDocumentBuilder::new(self)
    }

    fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, C, D, No> {
        requests::QueryDocumentsBuilder::new(self)
    }

    //fn with_stored_procedure<'c>(
    //    &'c self,
    //    stored_procedure_name: &'c dyn StoredProcedureName,
    //) -> StoredProcedureClient<'c, CUB> {
    //    StoredProcedureClient::new(&self, stored_procedure_name)
    //}

    //fn with_user_defined_function<'c>(
    //    &'c self,
    //    user_defined_function_name: &'c dyn UserDefinedFunctionName,
    //) -> UserDefinedFunctionClient<'c, CUB> {
    //    UserDefinedFunctionClient::new(&self, user_defined_function_name)
    //}

    //fn with_trigger<'c>(&'c self, trigger_name: &'c dyn TriggerName) -> TriggerClient<'c, CUB> {
    //    TriggerClient::new(&self, trigger_name)
    //}

    //fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, CUB> {
    //    requests::ListStoredProceduresBuilder::new(self)
    //}

    //fn list_user_defined_functions(
    //    &self,
    //) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, CUB> {
    //    requests::ListUserDefinedFunctionsBuilder::new(self)
    //}

    //fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, CUB> {
    //    requests::ListTriggersBuilder::new(self)
    //}

    //fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, CUB> {
    //    requests::GetPartitionKeyRangesBuilder::new(self)
    //}
}

impl<C, D> IntoDocumentClient<C, D, Self, DocumentStruct<C, D, Self>> for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn with_document(
        self,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<C, D, Self> {
        DocumentStruct::new(self, document_name, partition_keys)
    }
}
