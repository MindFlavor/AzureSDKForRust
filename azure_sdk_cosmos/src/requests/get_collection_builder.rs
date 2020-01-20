use crate::clients::{CollectionClient, CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::GetCollectionResponse;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> GetCollectionBuilder<'a, CUB> {
        GetCollectionBuilder {
            collection_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB> CollectionClientRequired<'a, CUB> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserAgentOption<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> ConsistencyLevelOption<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level
    }
}

impl<'a, CUB> UserAgentSupport<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetCollectionBuilder<'a, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetCollectionBuilder<'a, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ConsistencyLevelSupport<'a> for GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetCollectionBuilder<'a, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        GetCollectionBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> GetCollectionBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<GetCollectionResponse, AzureError> {
        trace!("GetCollectionResponse::execute called");

        let mut request = self.collection_client().main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name()
            ),
            hyper::Method::GET,
            ResourceType::Collections,
        );

        UserAgentOption::add_header(self, &mut request);
        ActivityIdOption::add_header(self, &mut request);
        ConsistencyLevelOption::add_header(self, &mut request);

        let request = request.body(hyper::Body::empty())?;

        let future_response = self.collection_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
