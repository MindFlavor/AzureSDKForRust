use crate::clients::CosmosUriBuilder;
use crate::prelude::*;
use crate::responses::CreateUserDefinedFunctionResponse;
use crate::UserDefinedFunctionClient;
use crate::UserDefinedFunctionClientRequired;
use crate::{UserDefinedFunctionBuilderTrait, UserDefinedFunctionTrait};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    user_defined_function_client: &'a UserDefinedFunctionClient<'a, CUB>,
    p_body: PhantomData<BodySet>,
    body: Option<&'a str>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> CreateUserDefinedFunctionBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        user_defined_function_client: &'a UserDefinedFunctionClient<'a, CUB>,
    ) -> CreateUserDefinedFunctionBuilder<'a, CUB, No> {
        CreateUserDefinedFunctionBuilder {
            user_defined_function_client,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB, BodySet> UserDefinedFunctionClientRequired<'a, CUB>
    for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_defined_function_client(&self) -> &'a UserDefinedFunctionClient<'a, CUB> {
        self.user_defined_function_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserDefinedFunctionBodyRequired<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, CUB, BodySet> UserAgentOption<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, BodySet> ActivityIdOption<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, BodySet> ConsistencyLevelOption<'a>
    for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB> UserDefinedFunctionBodySupport<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = CreateUserDefinedFunctionBuilder<'a, CUB, Yes>;

    #[inline]
    fn with_body(self, body: &'a str) -> Self::O {
        CreateUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            p_body: PhantomData {},
            body: Some(body),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, BodySet> UserAgentSupport<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, BodySet> ActivityIdSupport<'a> for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, BodySet> ConsistencyLevelSupport<'a>
    for CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateUserDefinedFunctionBuilder<'a, CUB, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        CreateUserDefinedFunctionBuilder {
            user_defined_function_client: self.user_defined_function_client,
            p_body: PhantomData {},
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> CreateUserDefinedFunctionBuilder<'a, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateUserDefinedFunctionResponse, AzureError> {
        trace!("CreateUserDefinedFunctionBuilder::execute called");

        let req = self
            .user_defined_function_client
            .prepare_request(hyper::Method::POST, false);

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Serialize)]
        struct Request<'a> {
            body: &'a str,
            id: &'a str,
        }
        let request = Request {
            body: self.body(),
            id: self
                .user_defined_function_client
                .user_defined_function_name()
                .name(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(hyper::Body::from(request))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.user_defined_function_client()
                .hyper_client()
                .request(request),
            StatusCode::CREATED,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
