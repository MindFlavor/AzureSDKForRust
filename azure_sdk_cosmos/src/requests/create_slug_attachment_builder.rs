use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::CreateSlugAttachmentResponse;
use crate::AttachmentBuilderTrait;
use crate::AttachmentClient;
use crate::AttachmentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    attachment_client: &'a AttachmentClient<'a, CUB>,
    p_body: PhantomData<BodySet>,
    body: Option<&'b [u8]>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> CreateSlugAttachmentBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a AttachmentClient<'a, CUB>,
    ) -> CreateSlugAttachmentBuilder<'a, 'b, CUB, No> {
        CreateSlugAttachmentBuilder {
            attachment_client,
            p_body: PhantomData {},
            body: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB, BodySet> AttachmentClientRequired<'a, CUB>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn attachment_client(&self) -> &'a AttachmentClient<'a, CUB> {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> BodyRequired<'b> for CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn body(&self) -> &'b [u8] {
        self.body.unwrap()
    }
}

impl<'a, 'b, CUB, BodySet> IfMatchConditionOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB, BodySet> UserAgentOption<'b> for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, BodySet> ActivityIdOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, BodySet> ConsistencyLevelOption<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB> BodySupport<'b> for CreateSlugAttachmentBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes>;

    #[inline]
    fn with_body(self, body: &'b [u8]) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            body: Some(body),
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet> IfMatchConditionSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            body: self.body,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet> UserAgentSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            body: self.body,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet> ActivityIdSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            body: self.body,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, BodySet> ConsistencyLevelSupport<'b>
    for CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>
where
    BodySet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateSlugAttachmentBuilder<'a, 'b, CUB, BodySet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateSlugAttachmentBuilder {
            attachment_client: self.attachment_client,
            p_body: PhantomData {},
            body: self.body,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> CreateSlugAttachmentBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateSlugAttachmentResponse, AzureError> {
        let mut req = self.attachment_client.prepare_request(hyper::Method::POST);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        req = req.header("Slug", self.attachment_client.attachment_name().name());

        let req = req.body(hyper::Body::from(self.body().to_owned()))?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.attachment_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
