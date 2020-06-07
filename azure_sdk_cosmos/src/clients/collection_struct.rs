use crate::clients::*;
use crate::requests;
use crate::{
    CollectionClient, CosmosClient, DatabaseClient, HasCosmosClient, HasDatabaseClient,
    HasHyperClient, IntoDocumentClient, IntoStoredProcedureClient, IntoTriggerClient,
    IntoUserDefinedFunctionClient, PartitionKeys, UserDefinedFunctionStruct, WithDocumentClient,
    WithStoredProcedureClient, WithTriggerClient, WithUserDefinedFunctionClient,
};
use azure_sdk_core::No;
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    database_client: Cow<'a, D>,
    collection_name: String,
    p_c: PhantomData<C>,
}

impl<'a, C, D> CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    pub(crate) fn new(database_client: Cow<'a, D>, collection_name: String) -> Self {
        Self {
            database_client,
            collection_name,
            p_c: PhantomData {},
        }
    }
}

impl<'a, C, D> HasHyperClient for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }
}

impl<'a, C, D> HasCosmosClient<C> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client.cosmos_client()
    }
}

impl<'a, C, D> HasDatabaseClient<C, D> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<'a, C, D> CollectionClient<C, D> for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
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

    fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, '_, C, D> {
        requests::ListStoredProceduresBuilder::new(self)
    }

    fn list_user_defined_functions(
        &self,
    ) -> requests::ListUserDefinedFunctionsBuilder<'_, '_, C, D> {
        requests::ListUserDefinedFunctionsBuilder::new(self)
    }

    fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, C, D> {
        requests::ListTriggersBuilder::new(self)
    }

    fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, C, D> {
        requests::GetPartitionKeyRangesBuilder::new(self)
    }
}

impl<C, D> IntoDocumentClient<C, D, Self, DocumentStruct<'static, C, D, Self>>
    for CollectionStruct<'static, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_document_client(
        self,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'static, C, D, Self> {
        DocumentStruct::new(Cow::Owned(self), document_name, partition_keys)
    }
}

impl<'a, C, D> WithDocumentClient<'a, C, D, Self, DocumentStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_document_client(
        &'a self,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'a, C, D, Self> {
        DocumentStruct::new(Cow::Borrowed(self), document_name, partition_keys)
    }
}

impl<'a, C, D> WithTriggerClient<'a, C, D, Self, TriggerStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_trigger_client(&'a self, trigger_name: String) -> TriggerStruct<'a, C, D, Self> {
        TriggerStruct::new(Cow::Borrowed(self), trigger_name)
    }
}

impl<'a, C, D> IntoTriggerClient<C, D, Self, TriggerStruct<'static, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_trigger_client(self, trigger_name: String) -> TriggerStruct<'static, C, D, Self> {
        TriggerStruct::new(Cow::Owned(self), trigger_name)
    }
}

impl<'a, C, D>
    WithUserDefinedFunctionClient<'a, C, D, Self, UserDefinedFunctionStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_user_defined_function_client(
        &'a self,
        user_defined_function_name: String,
    ) -> UserDefinedFunctionStruct<'a, C, D, Self> {
        UserDefinedFunctionStruct::new(Cow::Borrowed(self), user_defined_function_name)
    }
}

impl<'a, C, D>
    IntoUserDefinedFunctionClient<C, D, Self, UserDefinedFunctionStruct<'static, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_user_defined_function_client(
        self,
        user_defined_function_name: String,
    ) -> UserDefinedFunctionStruct<'static, C, D, Self> {
        UserDefinedFunctionStruct::new(Cow::Owned(self), user_defined_function_name)
    }
}

impl<'a, C, D> WithStoredProcedureClient<'a, C, D, Self, StoredProcedureStruct<'a, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_stored_procedure_client(
        &'a self,
        stored_procedure_name: String,
    ) -> StoredProcedureStruct<'a, C, D, Self> {
        StoredProcedureStruct::new(Cow::Borrowed(self), stored_procedure_name)
    }
}

impl<'a, C, D> IntoStoredProcedureClient<C, D, Self, StoredProcedureStruct<'static, C, D, Self>>
    for CollectionStruct<'a, C, D>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_stored_procedure_client(
        self,
        stored_procedure_name: String,
    ) -> StoredProcedureStruct<'static, C, D, Self> {
        StoredProcedureStruct::new(Cow::Owned(self), stored_procedure_name)
    }
}
