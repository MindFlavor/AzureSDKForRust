use crate::clients::CosmosUriBuilder;
use crate::prelude::*;
use crate::responses::CreateTriggerResponse;
use crate::trigger::*;
use crate::TriggerClient;
use crate::TriggerClientRequired;
use crate::{TriggerBuilderTrait, TriggerTrait};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    trigger_client: &'a TriggerClient<'a, CUB>,
    p_trigger_operation: PhantomData<TriggerOperationSet>,
    p_trigger_type: PhantomData<TriggerTypeSet>,
    p_body: PhantomData<BodySet>,
    trigger_operation: TriggerOperation,
    trigger_type: TriggerType,
    body: Option<&'a str>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> CreateTriggerBuilder<'a, CUB, No, No, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        trigger_client: &'a TriggerClient<'a, CUB>,
    ) -> CreateTriggerBuilder<'a, CUB, No, No, No> {
        CreateTriggerBuilder {
            trigger_client,
            p_trigger_operation: PhantomData {},
            trigger_operation: TriggerOperation::All,
            p_trigger_type: PhantomData {},
            trigger_type: TriggerType::Pre,
            p_body: PhantomData {},
            body: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> TriggerClientRequired<'a, CUB>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_client(&self) -> &'a TriggerClient<'a, CUB> {
        self.trigger_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB, TriggerTypeSet, BodySet> TriggerOperationRequired
    for CreateTriggerBuilder<'a, CUB, Yes, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_operation(&self) -> TriggerOperation {
        self.trigger_operation
    }
}

impl<'a, CUB, TriggerOperationSet, BodySet> TriggerTypeRequired
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, Yes, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn trigger_type(&self) -> TriggerType {
        self.trigger_type
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet> TriggerBodyRequired<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, Yes>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'a str {
        self.body.unwrap()
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentOption<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdOption<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelOption<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB, TriggerTypeSet, BodySet> TriggerOperationSupport
    for CreateTriggerBuilder<'a, CUB, No, TriggerTypeSet, BodySet>
where
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, Yes, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_trigger_operation(self, trigger_operation: TriggerOperation) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, BodySet> TriggerTypeSupport
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, No, BodySet>
where
    TriggerOperationSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, TriggerOperationSet, Yes, BodySet>;

    #[inline]
    fn with_trigger_type(self, trigger_type: TriggerType) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet> TriggerBodySupport<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, No>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, Yes>;

    #[inline]
    fn with_body(self, body: &'a str) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: Some(body),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> UserAgentSupport<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ActivityIdSupport<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet> ConsistencyLevelSupport<'a>
    for CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>
where
    TriggerOperationSet: ToAssign,
    TriggerTypeSet: ToAssign,
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateTriggerBuilder<'a, CUB, TriggerOperationSet, TriggerTypeSet, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        CreateTriggerBuilder {
            trigger_client: self.trigger_client,
            p_trigger_operation: PhantomData {},
            p_trigger_type: PhantomData {},
            p_body: PhantomData {},
            trigger_operation: self.trigger_operation,
            trigger_type: self.trigger_type,
            body: self.body,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> CreateTriggerBuilder<'a, CUB, Yes, Yes, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateTriggerResponse, AzureError> {
        trace!("CreateTriggerBuilder::execute called");

        let req = self
            .trigger_client
            .prepare_request(hyper::Method::POST, false);

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        struct _Request<'a> {
            pub id: &'a str,
            #[serde(rename = "triggerOperation")]
            pub trigger_operation: TriggerOperation,
            #[serde(rename = "triggerType")]
            pub trigger_type: TriggerType,
            pub body: &'a str,
        }

        let request = _Request {
            id: self.trigger_client.trigger_name().name(),
            trigger_operation: self.trigger_operation(),
            trigger_type: self.trigger_type(),
            body: self.body(),
        };

        let request = serde_json::to_string(&request)?;
        let request = req.body(hyper::Body::from(request))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.trigger_client().hyper_client().request(request),
            StatusCode::CREATED,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
