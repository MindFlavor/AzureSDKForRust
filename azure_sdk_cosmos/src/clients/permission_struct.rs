use crate::requests;
use crate::traits::*;
//use azure_sdk_core::No;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    user_client: USER,
    permission_name: String,
    p_c: PhantomData<C>,
    p_d: PhantomData<D>,
}

impl<C, D, USER> PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    pub(crate) fn new(user_client: USER, permission_name: String) -> Self {
        Self {
            user_client,
            permission_name,
            p_c: PhantomData {},
            p_d: PhantomData {},
        }
    }
}

impl<C, D, USER> HasHyperClient for PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.user_client.hyper_client()
    }
}

impl<C, D, USER> HasCosmosClient<C> for PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn cosmos_client(&self) -> &C {
        self.user_client.cosmos_client()
    }
}

impl<C, D, USER> HasDatabaseClient<C, D> for PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn database_client(&self) -> &D {
        self.user_client.database_client()
    }
}

impl<C, D, USER> HasUserClient<C, D, USER> for PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    #[inline]
    fn user_client(&self) -> &USER {
        &self.user_client
    }
}

impl<C, D, USER> PermissionClient<C, D, USER> for PermissionStruct<C, D, USER>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    USER: UserClient<C, D>,
{
    fn permission_name(&self) -> &str {
        &self.permission_name
    }

    fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_, C, D, USER> {
        requests::CreatePermissionBuilder::new(self)
    }

    //fn replace_permission<R>(&self) -> requests::ReplacePermissionBuilder<'_, CUB, R, No>
    //where
    //    R: PermissionResource,
    //{
    //    requests::ReplacePermissionBuilder::new(self)
    //}

    //fn get_permission(&self) -> requests::GetPermissionBuilder<'_, CUB> {
    //    requests::GetPermissionBuilder::new(self)
    //}

    //fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, CUB> {
    //    requests::DeletePermissionsBuilder::new(self)
    //}
}
