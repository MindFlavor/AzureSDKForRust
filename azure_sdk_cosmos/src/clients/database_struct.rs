use crate::clients::{CollectionStruct, UserStruct};
use crate::traits::*;
use crate::{requests, CosmosClient};
use azure_sdk_core::No;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    cosmos_client: Cow<'a, C>,
    database_name: String,
}

impl<'a, C> DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    pub(crate) fn new(cosmos_client: Cow<'a, C>, database_name: String) -> Self {
        DatabaseStruct {
            cosmos_client,
            database_name,
        }
    }
}

impl<'a, C> HasHyperClient for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }
}

impl<'a, C> HasCosmosClient<C> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        &self.cosmos_client
    }
}

impl<'a, C> DatabaseClient<C> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    #[inline]
    fn database_name(&self) -> &str {
        &self.database_name
    }

    fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, C> {
        requests::ListCollectionsBuilder::new(self)
    }

    fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_, C> {
        requests::GetDatabaseBuilder::new(self)
    }

    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, C> {
        requests::DeleteDatabaseBuilder::new(self)
    }

    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, C, No, No, No, No> {
        requests::CreateCollectionBuilder::new(self)
    }

    fn list_users(&self) -> requests::ListUsersBuilder<'_, '_, C> {
        requests::ListUsersBuilder::new(self)
    }
}

impl<'a, C> WithCollectionClient<'a, C, Self, CollectionStruct<'a, C, Self>>
    for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn with_collection_client(&'a self, collection_name: String) -> CollectionStruct<'a, C, Self> {
        CollectionStruct::new(Cow::Borrowed(self), collection_name)
    }
}

impl<'a, C> IntoCollectionClient<C, Self, CollectionStruct<'static, C, Self>>
    for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn into_collection_client(self, collection_name: String) -> CollectionStruct<'static, C, Self> {
        CollectionStruct::new(Cow::Owned(self), collection_name)
    }
}

impl<'a, C> WithUserClient<'a, C, Self, UserStruct<'a, C, Self>> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn with_user_client(&'a self, user_name: String) -> UserStruct<'a, C, Self> {
        UserStruct::new(Cow::Borrowed(self), user_name)
    }
}

impl<'a, C> IntoUserClient<C, Self, UserStruct<'static, C, Self>> for DatabaseStruct<'a, C>
where
    C: CosmosClient + Clone,
{
    fn into_user_client(self, user_name: String) -> UserStruct<'static, C, Self> {
        UserStruct::new(Cow::Owned(self), user_name)
    }
}
