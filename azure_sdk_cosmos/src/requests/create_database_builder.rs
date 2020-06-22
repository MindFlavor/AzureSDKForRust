use crate::prelude::*;
use crate::responses::CreateDatabaseResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use hyper::StatusCode;
use std::convert::TryInto;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    cosmos_client: &'b CosmosClient<'a, CUB>,
    p_database_name: PhantomData<DatabaseNameSet>,
    database_name: Option<&'b dyn DatabaseName>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> CreateDatabaseBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        cosmos_client: &'b CosmosClient<'a, CUB>,
    ) -> CreateDatabaseBuilder<'a, 'b, CUB, No> {
        CreateDatabaseBuilder {
            cosmos_client,
            p_database_name: PhantomData {},
            database_name: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> CosmosClientRequired<'a, 'b, CUB>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn cosmos_client(&self) -> &'b CosmosClient<'a, CUB> {
        self.cosmos_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> DatabaseNameRequired<'b> for CreateDatabaseBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'b dyn DatabaseName {
        self.database_name.unwrap()
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> UserAgentOption<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> ActivityIdOption<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> ConsistencyLevelOption<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> DatabaseNameSupport<'b> for CreateDatabaseBuilder<'a, 'b, CUB, No>
where
    CUB: CosmosUriBuilder,
{
    type O = CreateDatabaseBuilder<'a, 'b, CUB, Yes>;

    fn with_database_name(self, database_name: &'b dyn DatabaseName) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: Some(database_name),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> UserAgentSupport<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> ActivityIdSupport<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB, DatabaseNameSet> ConsistencyLevelSupport<'b>
    for CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>
where
    DatabaseNameSet: ToAssign,
    CUB: CosmosUriBuilder,
{
    type O = CreateDatabaseBuilder<'a, 'b, CUB, DatabaseNameSet>;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        CreateDatabaseBuilder {
            cosmos_client: self.cosmos_client,
            p_database_name: PhantomData {},
            database_name: self.database_name,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> CreateDatabaseBuilder<'a, 'b, CUB, Yes>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<CreateDatabaseResponse, AzureError> {
        trace!("CreateDatabaseBuilder::execute called");

        #[derive(Serialize, Debug)]
        struct CreateDatabaseRequest<'a> {
            pub id: &'a str,
        }

        let req = serde_json::to_string(&CreateDatabaseRequest {
            id: self.database_name().name(),
        })?;

        let request = self.cosmos_client().prepare_request(
            "dbs",
            hyper::Method::POST,
            crate::ResourceType::Databases,
        );

        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);

        let request = request.body(hyper::Body::from(req))?; // todo: set content-length here and elsewhere without builders

        debug!("create database request prepared == {:?}", request);

        let future_response = self.cosmos_client().hyper_client().request(request);
        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::CREATED).await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
