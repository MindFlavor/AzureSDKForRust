use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::database::DatabaseName;
use crate::traits::*;
use crate::{requests, CosmosClient, DatabaseClientRequestPreparer, UserName};
use crate::{CollectionName, DatabaseTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct DatabaseStruct<C>
where
    C: CosmosClient,
{
    cosmos_client: C,
    database_name: String,
}

impl<C> DatabaseStruct<C>
where
    C: CosmosClient,
{
    pub(crate) fn new(cosmos_client: C, database_name: String) -> Self {
        DatabaseStruct {
            cosmos_client,
            database_name,
        }
    }
}

impl<C> HasCosmosClient<C> for DatabaseStruct<C>
where
    C: CosmosClient,
{
    fn cosmos_client(&self) -> &C {
        &self.cosmos_client
    }
}

impl<C> DatabaseClient<C> for DatabaseStruct<C>
where
    C: CosmosClient,
{
    fn database_name(&self) -> &str {
        &self.database_name
    }

    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, C> {
        requests::ListCollectionsBuilder::new(self)
    }

    //fn get_database(&self) -> requests::GetDatabaseBuilder<'_, CUB> {
    //    requests::GetDatabaseBuilder::new(self)
    //}

    //fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, CUB> {
    //    requests::DeleteDatabaseBuilder::new(self)
    //}

    //fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, CUB, No, No, No, No> {
    //    requests::CreateCollectionBuilder::new(self)
    //}

    //fn with_collection<'c>(
    //    &'c self,
    //    collection_name: &'c dyn CollectionName,
    //) -> CollectionClient<'c, CUB> {
    //    CollectionClient::new(self, collection_name)
    //}

    //fn with_user<'c>(&'c self, user_name: &'c dyn UserName) -> UserClient<'c, CUB> {
    //    UserClient::new(&self, user_name)
    //}

    //fn list_users(&self) -> requests::ListUsersBuilder<'_, CUB> {
    //    requests::ListUsersBuilder::new(self)
    //}
}

impl<C> DatabaseClientRequestPreparer<C> for DatabaseStruct<C>
where
    C: CosmosClient,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name),
            method,
            ResourceType::Databases,
        )
    }
}
