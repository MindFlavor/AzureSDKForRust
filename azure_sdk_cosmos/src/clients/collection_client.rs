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
pub struct CollectionClient<'c, 'db, 'coll> {
    database_client: Cow<'coll, DatabaseClient<'c, 'db>>,
    collection_name: Cow<'coll, str>,
    p_c: PhantomData<C>,
}

impl<'c, 'db, 'coll> CollectionClient<'c, 'db, 'coll> {
    #[inline]
    pub(crate) fn new(
        database_client: Cow<'coll, DatabaseClient<'c, 'db>>,
        collection_name: Cow<'coll, str>,
    ) -> Self {
        Self {
            database_client,
            collection_name,
            p_c: PhantomData {},
        }
    }
}

impl<'coll, C, D> HasHyperClient for CollectionClient<'c, 'db, 'coll>
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

impl<'coll, C, D> HasCosmosClient<C> for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client.cosmos_client()
    }
}

impl<'coll, C, D> HasDatabaseClient<C, D> for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<'coll, C, D> CollectionClient<C, D> for CollectionClient<'c, 'db, 'coll>
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

impl<'coll, 'b, C, D> IntoDocumentClient<'b, C, D, Self, DocumentStruct<'coll, 'b, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_document_client<DocName>(
        self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'coll, 'b, C, D, Self>
    where
        DocName: Into<Cow<'b, str>>,
    {
        DocumentStruct::new(Cow::Owned(self), document_name.into(), partition_keys)
    }
}

impl<'coll, 'b, C, D>
    WithDocumentClient<'coll, 'b, C, D, Self, DocumentStruct<'coll, 'b, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_document_client<DocName>(
        &'coll self,
        document_name: DocName,
        partition_keys: PartitionKeys,
    ) -> DocumentStruct<'coll, 'b, C, D, Self>
    where
        DocName: Into<Cow<'b, str>>,
    {
        DocumentStruct::new(Cow::Borrowed(self), document_name.into(), partition_keys)
    }
}

impl<'coll, C, D> WithTriggerClient<'coll, C, D, Self, TriggerStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_trigger_client<IntoCowStr>(
        &'coll self,
        trigger_name: IntoCowStr,
    ) -> TriggerStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        TriggerStruct::new(Cow::Borrowed(self), trigger_name.into())
    }
}

impl<'coll, C, D> IntoTriggerClient<'coll, C, D, Self, TriggerStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_trigger_client<IntoCowStr>(
        self,
        trigger_name: IntoCowStr,
    ) -> TriggerStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        TriggerStruct::new(Cow::Owned(self), trigger_name.into())
    }
}

impl<'coll, C, D>
    WithUserDefinedFunctionClient<'coll, C, D, Self, UserDefinedFunctionStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_user_defined_function_client<IntoCowStr>(
        &'coll self,
        user_defined_function_name: IntoCowStr,
    ) -> UserDefinedFunctionStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        UserDefinedFunctionStruct::new(Cow::Borrowed(self), user_defined_function_name.into())
    }
}

impl<'coll, C, D>
    IntoUserDefinedFunctionClient<'coll, C, D, Self, UserDefinedFunctionStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_user_defined_function_client<IntoCowStr>(
        self,
        user_defined_function_name: IntoCowStr,
    ) -> UserDefinedFunctionStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        UserDefinedFunctionStruct::new(Cow::Owned(self), user_defined_function_name.into())
    }
}

impl<'coll, C, D>
    WithStoredProcedureClient<'coll, C, D, Self, StoredProcedureStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn with_stored_procedure_client<IntoCowStr>(
        &'coll self,
        stored_procedure_name: IntoCowStr,
    ) -> StoredProcedureStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        StoredProcedureStruct::new(Cow::Borrowed(self), stored_procedure_name.into())
    }
}

impl<'coll, C, D>
    IntoStoredProcedureClient<'coll, C, D, Self, StoredProcedureStruct<'coll, C, D, Self>>
    for CollectionClient<'c, 'db, 'coll>
where
    C: CosmosClient + Clone,
    D: DatabaseClient<C> + Clone,
{
    fn into_stored_procedure_client<IntoCowStr>(
        self,
        stored_procedure_name: IntoCowStr,
    ) -> StoredProcedureStruct<'coll, C, D, Self>
    where
        IntoCowStr: Into<Cow<'coll, str>>,
    {
        StoredProcedureStruct::new(Cow::Owned(self), stored_procedure_name.into())
    }
}
