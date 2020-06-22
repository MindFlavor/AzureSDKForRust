use crate::clients::CosmosUriBuilder;
use crate::{requests, CosmosClient};
use azure_sdk_core::No;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct DatabaseClient<'c, 'db, CUB>
where
    CUB: CosmosUriBuilder,
{
    cosmos_client: Cow<'db, CosmosClient<'c, CUB>>,
    database_name: Cow<'db, str>,
}

impl<'c, 'db, CUB> DatabaseClient<'c, 'db, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        cosmos_client: Cow<'db, CosmosClient<'c, CUB>>,
        database_name: Cow<'db, str>,
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
    pub fn cosmos_client(&self) -> &CosmosClient<'c, CUB> {
        &self.cosmos_client
    }

    #[inline]
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn list_collections<'a>(&'a self) -> requests::ListCollectionsBuilder<'a, 'c, 'db, CUB> {
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
    //    &'db self,
    //    collection_name: IntoCowStr,
    //) -> CollectionStruct<'db, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'db, str>>,
    //{
    //    CollectionStruct::new(Cow::Borrowed(self), collection_name.into())
    //}

    //fn into_collection_client<IntoCowStr>(
    //    self,
    //    collection_name: IntoCowStr,
    //) -> CollectionStruct<'db, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'db, str>>,
    //{
    //    CollectionStruct::new(Cow::Owned(self), collection_name.into())
    //}

    //fn with_user_client<IntoCowStr>(&'db self, user_name: IntoCowStr) -> UserStruct<'db, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'db, str>>,
    //{
    //    UserStruct::new(Cow::Borrowed(self), user_name.into())
    //}

    //fn into_user_client<IntoCowStr>(self, user_name: IntoCowStr) -> UserStruct<'db, C, Self>
    //where
    //    IntoCowStr: Into<Cow<'db, str>>,
    //{
    //    UserStruct::new(Cow::Owned(self), user_name.into())
    //}
}
