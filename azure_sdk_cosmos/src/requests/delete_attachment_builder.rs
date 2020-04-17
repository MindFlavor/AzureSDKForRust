use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
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
pub struct DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    attachment_client: &'a AttachmentClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        attachment_client: &'a AttachmentClient<'a, CUB>,
    ) -> DeleteAttachmentBuilder<'a, 'b, CUB> {
        DeleteAttachmentBuilder {
            attachment_client,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB> AttachmentClientRequired<'a, CUB> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn attachment_client(&self) -> &'a AttachmentClient<'a, CUB> {
        self.attachment_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteAttachmentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        DeleteAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteAttachmentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        DeleteAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteAttachmentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        DeleteAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = DeleteAttachmentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        DeleteAttachmentBuilder {
            attachment_client: self.attachment_client,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> DeleteAttachmentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<crate::responses::DeleteAttachmentResponse, AzureError> {
        let mut req = self
            .attachment_client
            .prepare_request(hyper::Method::DELETE);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(
            self.attachment_client.document_client().partition_keys(),
            req,
        );

        let req = req.body(hyper::Body::empty())?;

        debug!("req == {:#?}", req);

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.attachment_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
