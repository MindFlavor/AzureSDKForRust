use crate::requests;
use crate::traits::*;
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    document_client: DOC,
    attachment_name: String,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
    p_coll: PhantomData<COLL>,
}

impl<C, D, COLL, DOC> AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    pub(crate) fn new(document_client: DOC, attachment_name: String) -> Self {
        Self {
            document_client,
            attachment_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
            p_coll: PhantomData {},
        }
    }
}

impl<C, D, COLL, DOC> HasHyperClient for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.document_client().hyper_client()
    }
}

impl<C, D, COLL, DOC> HasCosmosClient<C> for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.document_client().cosmos_client()
    }
}

impl<C, D, COLL, DOC> HasDatabaseClient<C, D> for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.document_client().database_client()
    }
}

impl<C, D, COLL, DOC> HasCollectionClient<C, D, COLL> for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn collection_client(&self) -> &COLL {
        self.document_client().collection_client()
    }
}

impl<C, D, COLL, DOC> HasDocumentClient<C, D, COLL, DOC> for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    #[inline]
    fn document_client(&self) -> &DOC {
        &self.document_client
    }
}

impl<C, D, COLL, DOC> AttachmentClient<C, D, COLL, DOC> for AttachmentStruct<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    //fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, CUB, No, No> {
    //    requests::CreateSlugAttachmentBuilder::new(self)
    //}

    //fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, CUB, No, No> {
    //    requests::ReplaceSlugAttachmentBuilder::new(self)
    //}

    //fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, CUB, No, No> {
    //    requests::CreateReferenceAttachmentBuilder::new(self)
    //}

    //fn replace_reference(
    //    &self,
    //) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, CUB, No, No> {
    //    requests::ReplaceReferenceAttachmentBuilder::new(self)
    //}

    //fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, CUB> {
    //    requests::DeleteAttachmentBuilder::new(self)
    //}

    //fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, CUB> {
    //    requests::GetAttachmentBuilder::new(self)
    //}
}
