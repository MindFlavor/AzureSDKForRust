use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ListAttachmentsResponse;
use crate::DocumentBuilderTrait;
use crate::DocumentClient;
use crate::DocumentClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    p_partition_keys: PhantomData<PartitionKeysSet>,
    partition_keys: Option<&'b PartitionKeys>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
    continuation: Option<&'b str>,
    max_item_count: i32,
    a_im: bool,
}

impl<'a, 'b, CUB> ListAttachmentsBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
    ) -> ListAttachmentsBuilder<'a, 'b, CUB, No> {
        ListAttachmentsBuilder {
            document_client,
            p_partition_keys: PhantomData {},
            partition_keys: None,
            if_match_condition: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            continuation: None,
            max_item_count: -1,
            a_im: false,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> DocumentClientRequired<'a, CUB>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_client(&self) -> &'a DocumentClient<'a, CUB> {
        self.document_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> PartitionKeysRequired<'b> for ListAttachmentsBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn partition_keys(&self) -> &'b PartitionKeys {
        self.partition_keys.unwrap()
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> IfMatchConditionOption<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> UserAgentOption<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ActivityIdOption<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ConsistencyLevelOption<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ContinuationOption<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn continuation(&self) -> Option<&'b str> {
        self.continuation
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> MaxItemCountOption
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn max_item_count(&self) -> i32 {
        self.max_item_count
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> AIMOption
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn a_im(&self) -> bool {
        self.a_im
    }
}

impl<'a, 'b, CUB> PartitionKeysSupport<'b> for ListAttachmentsBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, Yes>;

    #[inline]
    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: Some(partition_keys),
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> IfMatchConditionSupport<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: Some(if_match_condition),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> UserAgentSupport<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ActivityIdSupport<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ConsistencyLevelSupport<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> ContinuationSupport<'b>
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_continuation(self, continuation: &'b str) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: Some(continuation),
            max_item_count: self.max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> MaxItemCountSupport
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_max_item_count(self, max_item_count: i32) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count,
            a_im: self.a_im,
        }
    }
}

impl<'a, 'b, CUB, PartitionKeysSet> AIMSupport
    for ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>
where
    PartitionKeysSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = ListAttachmentsBuilder<'a, 'b, CUB, PartitionKeysSet>;

    #[inline]
    fn with_a_im(self, a_im: bool) -> Self::O {
        ListAttachmentsBuilder {
            document_client: self.document_client,
            p_partition_keys: PhantomData {},
            partition_keys: self.partition_keys,
            if_match_condition: self.if_match_condition,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
            continuation: self.continuation,
            max_item_count: self.max_item_count,
            a_im,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> ListAttachmentsBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListAttachmentsResponse, AzureError> {
        let mut req = self.document_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.document_client.database_name().name(),
                self.document_client.collection_name().name(),
                self.document_client.document_name().name()
            ),
            hyper::Method::GET,
            ResourceType::Attachments,
        );

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = ContinuationOption::add_header(self, req);
        req = MaxItemCountOption::add_header(self, req);
        req = AIMOption::add_header(self, req);
        req = PartitionKeysRequired::add_header(self, req);

        let req = req.body(hyper::Body::empty())?;

        let (headers, whole_body) = check_status_extract_headers_and_body(
            self.document_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        Ok((&headers, &whole_body as &[u8]).try_into()?)
    }
}
