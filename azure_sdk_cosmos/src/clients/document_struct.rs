use crate::requests;
use crate::{
    AttachmentStruct, CollectionClient, CosmosClient, DatabaseClient, DocumentClient,
    HasCollectionClient, HasCosmosClient, HasDatabaseClient, HasHyperClient, IntoAttachmentClient,
    PartitionKeys,
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

    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, C, D, COLL> {
        requests::ListAttachmentsBuilder::new(self)
    }
}

impl<C, D, COLL> IntoAttachmentClient<C, D, COLL, Self, AttachmentStruct<C, D, COLL, Self>>
    for DocumentStruct<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn with_attachment(self, attachment_name: String) -> AttachmentStruct<C, D, COLL, Self> {
        AttachmentStruct::new(self, attachment_name)
    }
}
