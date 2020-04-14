use crate::attachment::AttachmentName;
use crate::clients::{Client, CosmosUriBuilder, DocumentClient, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::DocumentTrait;
use crate::{AttachmentBuilderTrait, AttachmentTrait};

#[derive(Debug, Clone)]
pub struct AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    attachment_name: &'a dyn AttachmentName,
}

impl<'a, CUB> AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
        attachment_name: &'a dyn AttachmentName,
    ) -> Self {
        AttachmentClient {
            document_client,
            attachment_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.document_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> AttachmentTrait<'a, CUB> for AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.document_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.document_client.collection_name()
    }

    fn document_name(&self) -> &'a dyn DocumentName {
        self.document_client.document_name()
    }

    fn attachment_name(&self) -> &'a dyn AttachmentName {
        self.attachment_name
    }

    //fn create_stored_procedure(&self) -> requests::CreateStoredProcedureBuilder<'_, CUB, No> {
    //    requests::CreateStoredProcedureBuilder::new(self)
    //}

    //fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, CUB, No> {
    //    requests::ReplaceStoredProcedureBuilder::new(self)
    //}

    //fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_, CUB> {
    //    requests::ExecuteStoredProcedureBuilder::new(self)
    //}

    //fn delete_stored_procedure(&self) -> requests::DeleteStoredProcedureBuilder<'_, CUB> {
    //    requests::DeleteStoredProcedureBuilder::new(self)
    //}
}

impl<'a, CUB> AttachmentBuilderTrait<'a, CUB> for AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_name().name(),
                self.collection_name().name(),
                self.document_name().name()
            ),
            method,
            ResourceType::StoredProcedures,
        )
    }
}
