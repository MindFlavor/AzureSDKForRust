use crate::clients::CosmosUriBuilder;
use crate::{requests, CosmosClient};
use azure_sdk_core::No;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    cosmos_client: Cow<'a, CosmosClient<'a, CUB>>,
    database_name: Cow<'a, str>,
}

impl<'a, CUB> DatabaseClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        cosmos_client: Cow<'a, CosmosClient<'a, CUB>>,
        database_name: Cow<'a, str>,
    ) -> Self {
        Self {
            cosmos_client,
            database_name,
        }
    }

    #[inline]
    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.cosmos_client().hyper_client()
    }

    #[inline]
    pub fn cosmos_client(&self) -> &CosmosClient<'a, CUB> {
        &self.cosmos_client
    }

    #[inline]
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn list_collections(&'a self) -> requests::ListCollectionsBuilder<'_, CUB> {
        requests::ListCollectionsBuilder::new(self)
    }

    //fn get_database(&self) -> requests::GetDatabaseBuilder<'_, '_, C> {
    //    requests::GetDatabaseBuilder::new(self)
    //}

    //fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, C> {
    //    requests::DeleteDatabaseBuilder::new(self)
    //}

    //fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, C, No, No, No, No> {
    //    requests::CreateCollectionBuilder::new(self)
    //}

    //fn list_users(&self) -> requests::ListUsersBuilder<'_, '_, C> {
    //    requests::ListUsersBuilder::new(self)
    //}

    //fn with_collection_client<IntoCowStr>(
    //    &'a self,
    //    collection_name: IntoCowStr,
    //) -> CollectionStruct<'a, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'a, str>>,
    //{
    //    CollectionStruct::new(Cow::Borrowed(self), collection_name.into())
    //}

    //fn into_collection_client<IntoCowStr>(
    //    self,
    //    collection_name: IntoCowStr,
    //) -> CollectionStruct<'a, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'a, str>>,
    //{
    //    CollectionStruct::new(Cow::Owned(self), collection_name.into())
    //}

    //fn with_user_client<IntoCowStr>(&'a self, user_name: IntoCowStr) -> UserStruct<'a, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'a, str>>,
    //{
    //    UserStruct::new(Cow::Borrowed(self), user_name.into())
    //}

    //fn into_user_client<IntoCowStr>(self, user_name: IntoCowStr) -> UserStruct<'a, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'a, str>>,
    //{
    //    UserStruct::new(Cow::Owned(self), user_name.into())
    //}
}
