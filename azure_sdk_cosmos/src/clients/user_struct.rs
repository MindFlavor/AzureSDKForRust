//use crate::prelude::*;
use crate::requests;
use crate::traits::*;
use crate::PermissionStruct;
use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    database_client: D,
    user_name: String,
    p_c: PhantomData<C>,
}

impl<C, D> UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    pub(crate) fn new(database_client: D, user_name: String) -> Self {
        Self {
            database_client,
            user_name,
            p_c: PhantomData {},
        }
    }
}

impl<C, D> HasHyperClient for UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.database_client().hyper_client()
    }
}

impl<C, D> HasCosmosClient<C> for UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.database_client().cosmos_client()
    }
}

impl<C, D> HasDatabaseClient<C, D> for UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    #[inline]
    fn database_client(&self) -> &D {
        &self.database_client
    }
}

impl<C, D> UserClient<C, D> for UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn user_name(&self) -> &str {
        &self.user_name
    }

    fn create_user(&self) -> requests::CreateUserBuilder<'_, '_, C, D> {
        requests::CreateUserBuilder::new(self)
    }

    fn get_user(&self) -> requests::GetUserBuilder<'_, '_, C, D> {
        requests::GetUserBuilder::new(self)
    }

    fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, '_, C, D, No> {
        requests::ReplaceUserBuilder::new(self)
    }

    fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_, C, D> {
        requests::DeleteUserBuilder::new(self)
    }

    //fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, CUB> {
    //    requests::ListPermissionsBuilder::new(self)
    //}
}

impl<C, D> IntoPermissionClient<C, D, Self, PermissionStruct<C, D, Self>> for UserStruct<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn with_permission(self, permission_name: String) -> PermissionStruct<C, D, Self> {
        PermissionStruct::new(self, permission_name)
    }
}
