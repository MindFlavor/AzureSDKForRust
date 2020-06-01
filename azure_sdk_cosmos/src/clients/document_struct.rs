use crate::attachment::AttachmentName;
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::CollectionTrait;
use crate::{
    CollectionClient, CosmosClient, DatabaseClient, DocumentBuilderTrait, DocumentClient,
    DocumentTrait, HasCollectionClient, HasCosmosClient, HasDatabaseClient, HasHyperClient,
    PartitionKeys, ResourceType,
};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    collection_client: COLL,
    document_name: String,
    partition_keys: PartitionKeys,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<C, D, COLL> DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    pub(crate) fn new(
        collection_client: COLL,
        document_name: String,
        partition_keys: PartitionKeys,
    ) -> Self {
        Self {
            collection_client,
            document_name,
            partition_keys,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<C, D, COLL> HasHyperClient for DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client().hyper_client()
    }
}

impl<C, D, COLL> HasCosmosClient<C> for DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.collection_client().cosmos_client()
    }
}

impl<C, D, COLL> HasDatabaseClient<C, D> for DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.collection_client().database_client()
    }
}

impl<C, D, COLL> HasCollectionClient<C, D, COLL> for DocumentStruct<C, D, COLL>
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

impl<C, D, COLL> DocumentClient<C, D, COLL> for DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn document_name(&self) -> &str {
        &self.document_name
    }

    fn partition_keys(&self) -> &PartitionKeys {
        &self.partition_keys
    }

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, C, D, COLL> {
        requests::GetDocumentBuilder::new(self)
    }

    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, C, D, COLL> {
        requests::DeleteDocumentBuilder::new(self)
    }

    //fn with_attachment(
    //    &'a self,
    //    attachment_name: &'a dyn AttachmentName,
    //) -> AttachmentClient<'_, CUB> {
    //    AttachmentClient::new(&self, attachment_name)
    //}

    //fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, CUB> {
    //    requests::ListAttachmentsBuilder::new(self)
    //}
}
