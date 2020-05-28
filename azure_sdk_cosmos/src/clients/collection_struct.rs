use crate::clients::*;
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::stored_procedure::StoredProcedureName;
use crate::trigger::TriggerName;
use crate::user_defined_function::UserDefinedFunctionName;
use crate::{
    CollectionBuilderTrait, CollectionClient, CollectionClientRequestPreparer, CollectionTrait,
    CosmosClient, DatabaseClient, DatabaseTrait, HasCosmosClient, HasDatabaseClient,
    HasHyperClient, PartitionKeys, ResourceType,
};
use azure_sdk_core::No;
use serde::Serialize;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    p_c: PhantomData<C>,
    database_client: D,
    collection_name: String,
}

impl<C, D> CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    pub(crate) fn new(database_client: D, collection_name: String) -> Self {
        Self {
            p_c: PhantomData {},
            database_client,
            collection_name,
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

    //fn get_collection(&self) -> requests::GetCollectionBuilder<'_, CUB> {
    //    requests::GetCollectionBuilder::new(self)
    //}

    //fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, CUB> {
    //    requests::DeleteCollectionBuilder::new(self)
    //}

    //fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, CUB, No, No> {
    //    requests::ReplaceCollectionBuilder::new(self)
    //}

    //fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB> {
    //    requests::ListDocumentsBuilder::new(self)
    //}

    //fn create_document<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No, No>
    //where
    //    T: Serialize,
    //{
    //    requests::CreateDocumentBuilder::new(self)
    //}

    //fn replace_document<T>(&self) -> requests::ReplaceDocumentBuilder<'_, '_, T, CUB, No, No, No>
    //where
    //    T: Serialize,
    //{
    //    requests::ReplaceDocumentBuilder::new(self)
    //}

    //fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, CUB, No> {
    //    requests::QueryDocumentsBuilder::new(self)
    //}

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

    //fn with_document<'c>(
    //    &'c self,
    //    document_name: &'c dyn DocumentName,
    //    partition_keys: &'c PartitionKeys,
    //) -> DocumentClient<'c, CUB> {
    //    DocumentClient::new(&self, document_name, partition_keys)
    //}
}

impl<C, D> CollectionClientRequestPreparer for CollectionStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.database_client().cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}",
                self.database_client().database_name(),
                self.collection_name()
            ),
            method,
            ResourceType::Collections,
        )
    }
}
