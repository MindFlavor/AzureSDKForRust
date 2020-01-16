use crate::clients::{Client, CosmosUriBuilder, ResourceType};
use crate::responses::ListDatabasesResponse;
use crate::ClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    client: &'a Client<CUB>,
}

impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(client: &'a Client<CUB>) -> ListDatabasesBuilder<'a, CUB> {
        ListDatabasesBuilder { client }
    }
}

impl<'a, CUB> ClientRequired<'a, CUB> for ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn client(&self) -> &'a Client<CUB> {
        self.client
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListDatabasesBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(self) -> Result<ListDatabasesResponse, AzureError> {
        trace!("ListDatabasesBuilder::execute called");

        let request = self
            .client
            .prepare_request("dbs", hyper::Method::GET, ResourceType::Databases)
            .body(hyper::Body::empty())?;

        let future_response = self.client.hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
