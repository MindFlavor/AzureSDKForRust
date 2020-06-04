use crate::requests;
use crate::{PartitionKeys, ResourceType};
use azure_sdk_core::No;
use http::request::Builder;
use hyper_rustls::HttpsConnector;
use std::fmt::Debug;

pub trait HasHyperClient: Debug {
    fn hyper_client(&self) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;
}

pub trait CosmosClient: HasHyperClient {
    fn create_database(&self) -> requests::CreateDatabaseBuilder<'_, No>;

    fn prepare_request(
        &self,
        uri_path: &str,
        http_method: hyper::Method,
        resource_type: ResourceType,
    ) -> Builder;
}

pub trait HasCosmosClient<C>: HasHyperClient
where
    C: CosmosClient,
{
    fn cosmos_client(&self) -> &C;
}

pub trait DatabaseClient<C>: HasCosmosClient<C>
where
    C: CosmosClient,
{
    fn database_name(&self) -> &str;
    fn list_collections(&self) -> crate::requests::ListCollectionsBuilder<'_, C>;
    fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, C, No, No, No, No>;
    fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, C>;

    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client()
            .prepare_request("dbs", method, ResourceType::Databases)
    }
    fn prepare_request_with_database_name(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}", self.database_name()),
            method,
            ResourceType::Databases,
        )
    }
}

pub trait HasDatabaseClient<C, D>: HasCosmosClient<C>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn database_client(&self) -> &D;
}

pub trait IntoDatabaseClient<C, D>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn with_database(self, database_name: String) -> D;
}

pub trait CollectionClient<C, D>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
{
    fn collection_name(&self) -> &str;

    fn get_collection(&self) -> requests::GetCollectionBuilder<'_, C, D>;
    fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, C, D>;
    fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, '_, C, D, No, No>;

    fn create_document(&self) -> requests::CreateDocumentBuilder<'_, '_, C, D, No>;

    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client().database_name()),
            method,
            ResourceType::Collections,
        )
    }
    fn prepare_request_with_collection_name(
        &self,
        method: hyper::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}",
                self.database_client().database_name(),
                self.collection_name()
            ),
            method,
            ResourceType::Collections,
        )
    }
}

pub trait HasCollectionClient<C, D, COLL>: HasDatabaseClient<C, D>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn collection_client(&self) -> &COLL;
}

pub trait IntoCollectionClient<C, D, COLL>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn with_collection(self, collection_name: String) -> COLL;
}

pub trait DocumentClient<C, D, COLL>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
{
    fn document_name(&self) -> &str;
    fn partition_keys(&self) -> &PartitionKeys;

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, C, D, COLL>;
    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, C, D, COLL>;
    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, C, D, COLL>;

    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.database_client().database_name(),
                self.collection_client().collection_name()
            ),
            method,
            ResourceType::Documents,
        )
    }
    fn prepare_request_with_document_name(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_name()
            ),
            method,
            ResourceType::Documents,
        )
    }
}

pub trait HasDocumentClient<C, D, COLL, DOC>: HasCollectionClient<C, D, COLL>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn document_client(&self) -> &DOC;
}

pub trait IntoDocumentClient<C, D, COLL, DOC>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn with_document(self, document_name: String, partition_keys: PartitionKeys) -> DOC;
}

pub trait AttachmentClient<C, D, COLL, DOC>: HasDocumentClient<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
{
    fn attachment_name(&self) -> &str;

    fn create_slug(&self)
        -> requests::CreateSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    fn replace_slug(
        &self,
    ) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    fn create_reference(
        &self,
    ) -> requests::CreateReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    fn replace_reference(
        &self,
    ) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, C, D, COLL, DOC, No, No>;
    fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, C, D, COLL, DOC>;
    fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, C, D, COLL, DOC>;

    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
            ),
            method,
            ResourceType::Attachments,
        )
    }
    fn prepare_request_with_attachment_name(
        &self,
        method: hyper::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
                self.attachment_name()
            ),
            method,
            ResourceType::Attachments,
        )
    }
}

pub trait HasAttachmentClient<C, D, COLL, DOC, ATT>: HasDocumentClient<C, D, COLL, DOC>
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
    ATT: AttachmentClient<C, D, COLL, DOC>,
{
    fn attachment_client(&self) -> &ATT;
}

pub trait IntoAttachmentClient<C, D, COLL, DOC, ATT>: Debug
where
    C: CosmosClient,
    D: DatabaseClient<C>,
    COLL: CollectionClient<C, D>,
    DOC: DocumentClient<C, D, COLL>,
    ATT: AttachmentClient<C, D, COLL, DOC>,
{
    fn with_attachment(self, attachment_name: String) -> ATT;
}

///////////////////////////////////////////////
//////
//// Old implementation
//////
///////////////////////////////////////////////
//pub trait CosmosTrait<CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    //fn list_databases(&self) -> requests::ListDatabasesBuilder<'_, CUB>;
//    //fn with_database<'d>(&'d self, database_name: &'d dyn DatabaseName) -> DatabaseClient<'d, CUB>;
//    //fn create_database<DB>(&self) -> requests::CreateDatabaseBuilder<'_, CUB, DB, No>
//    //where
//    //    DB: DatabaseName;
//}
//
//pub trait DatabaseTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    //fn list_collections(&self) -> requests::ListCollectionsBuilder<'_, CUB>;
//    //fn get_database(&self) -> requests::GetDatabaseBuilder<'_, CUB>;
//    //fn delete_database(&self) -> requests::DeleteDatabaseBuilder<'_, CUB>;
//    //fn create_collection(&self) -> requests::CreateCollectionBuilder<'_, CUB, No, No, No, No>;
//    //fn with_collection<'c>(
//    //    &'c self,
//    //    collection_name: &'c dyn CollectionName,
//    //) -> CollectionClient<'c, CUB>;
//    //fn with_user<'c>(&'c self, user_name: &'c dyn UserName) -> UserClient<'c, CUB>;
//    //fn list_users(&self) -> requests::ListUsersBuilder<'_, CUB>;
//}
//
//pub trait CollectionTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    //fn get_collection(&self) -> requests::GetCollectionBuilder<'_, CUB>;
//    //fn delete_collection(&self) -> requests::DeleteCollectionBuilder<'_, CUB>;
//    //fn replace_collection(&self) -> requests::ReplaceCollectionBuilder<'_, CUB, No, No>;
//    //fn list_documents(&self) -> requests::ListDocumentsBuilder<'_, '_, CUB>;
//    //fn create_document<T>(&self) -> requests::CreateDocumentBuilder<'_, '_, T, CUB, No, No>
//    //where
//    //    T: Serialize;
//    //fn replace_document<T>(&self) -> requests::ReplaceDocumentBuilder<'_, '_, T, CUB, No, No, No>
//    //where
//    //    T: Serialize;
//    //fn query_documents(&self) -> requests::QueryDocumentsBuilder<'_, '_, CUB, No>;
//    //fn with_stored_procedure<'c>(
//    //    &'c self,
//    //    stored_procedure_name: &'c dyn StoredProcedureName,
//    //) -> StoredProcedureClient<'c, CUB>;
//    //fn with_user_defined_function<'c>(
//    //    &'c self,
//    //    user_defined_function_name: &'c dyn UserDefinedFunctionName,
//    //) -> UserDefinedFunctionClient<'c, CUB>;
//    //fn with_trigger<'c>(&'c self, trigger_name: &'c dyn TriggerName) -> TriggerClient<'c, CUB>;
//    //fn list_stored_procedures(&self) -> requests::ListStoredProceduresBuilder<'_, CUB>;
//    //fn list_user_defined_functions(&self)
//    //    -> requests::ListUserDefinedFunctionsBuilder<'_, '_, CUB>;
//    //fn list_triggers(&self) -> requests::ListTriggersBuilder<'_, '_, CUB>;
//    //fn get_partition_key_ranges(&self) -> requests::GetPartitionKeyRangesBuilder<'_, '_, CUB>;
//    //fn with_document<'c>(
//    //    &'c self,
//    //    document_name: &'c dyn DocumentName,
//    //    partition_keys: &'c PartitionKeys,
//    //) -> DocumentClient<'c, CUB>;
//}
//
//pub trait DocumentTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    fn document_name(&self) -> &'a dyn DocumentName;
//    fn partition_keys(&self) -> &'a PartitionKeys;
//    //fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, CUB>;
//    //fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, CUB>;
//    //fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, CUB>;
//    //fn with_attachment(
//    //    &'a self,
//    //    attachment_name: &'a dyn AttachmentName,
//    //) -> AttachmentClient<'_, CUB>;
//}
//
//pub trait StoredProcedureTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    fn stored_procedure_name(&self) -> &'a dyn StoredProcedureName;
//    //fn create_stored_procedure(&self) -> requests::CreateStoredProcedureBuilder<'_, CUB, No>;
//    //fn replace_stored_procedure(&self) -> requests::ReplaceStoredProcedureBuilder<'_, CUB, No>;
//    //fn execute_stored_procedure(&self) -> requests::ExecuteStoredProcedureBuilder<'_, '_, CUB>;
//    //fn delete_stored_procedure(&self) -> requests::DeleteStoredProcedureBuilder<'_, CUB>;
//}
//
//pub trait UserDefinedFunctionTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    fn user_defined_function_name(&self) -> &'a dyn UserDefinedFunctionName;
//    //fn create_user_defined_function(
//    //    &self,
//    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No>;
//    //fn replace_user_defined_function(
//    //    &self,
//    //) -> requests::CreateOrReplaceUserDefinedFunctionBuilder<'_, CUB, No>;
//    //fn delete_user_defined_function(&self) -> requests::DeleteUserDefinedFunctionBuilder<'_, CUB>;
//}
//
//pub trait TriggerTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    fn trigger_name(&self) -> &'a dyn TriggerName;
//    //fn create_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No>;
//    //fn replace_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No>;
//    //fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, CUB>;
//}
//
//pub trait AttachmentTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn collection_name(&self) -> &'a dyn CollectionName;
//    fn document_name(&self) -> &'a dyn DocumentName;
//    fn attachment_name(&self) -> &'a dyn AttachmentName;
//    //fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, CUB, No, No>;
//    //fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, CUB, No, No>;
//    //fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
//    //fn replace_reference(&self)
//    //    -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, CUB, No, No>;
//    //fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, CUB>;
//    //fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, CUB>;
//}
//
//pub trait UserTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn user_name(&self) -> &'a dyn UserName;
//    //fn create_user(&self) -> requests::CreateUserBuilder<'_, CUB>;
//    //fn get_user(&self) -> requests::GetUserBuilder<'_, CUB>;
//    //fn replace_user(&self) -> requests::ReplaceUserBuilder<'_, CUB, No>;
//    //fn delete_user(&self) -> requests::DeleteUserBuilder<'_, CUB>;
//    //fn with_permission<'c>(
//    //    &'c self,
//    //    permission_name: &'c dyn PermissionName,
//    //) -> PermissionClient<'c, CUB>;
//    //fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, CUB>;
//}
//
//pub trait PermissionTrait<'a, CUB>
//where
//    CUB: CosmosUriBuilder,
//{
//    fn database_name(&self) -> &'a dyn DatabaseName;
//    fn user_name(&self) -> &'a dyn UserName;
//    fn permission_name(&self) -> &'a dyn PermissionName;
//    //fn create_permission<R>(&self) -> requests::CreatePermissionBuilder<'_, CUB, R, No>
//    //where
//    //    R: PermissionResource;
//    //fn replace_permission<R>(&self) -> requests::ReplacePermissionBuilder<'_, CUB, R, No>
//    //where
//    //    R: PermissionResource;
//    //fn get_permission(&self) -> requests::GetPermissionBuilder<'_, CUB>;
//    //fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, CUB>;
//}
