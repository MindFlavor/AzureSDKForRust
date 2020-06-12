#![recursion_limit = "128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate quick_error;
pub mod key_client;
mod rest_client;
pub use self::rest_client::{
    get_default_json_mime, get_json_mime_fullmetadata, get_json_mime_nometadata, perform_request,
    ServiceType,
};
use crate::key_client::KeyClient;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::headers::COPY_ID;
use azure_sdk_core::util::HeaderMapExt;
mod into_azure_path;
pub mod prelude;
pub use self::into_azure_path::IntoAzurePath;
mod blob_sas_builder;
use http::HeaderMap;
use hyper::{self, Method};
mod connection_string;
mod connection_string_builder;
pub use self::connection_string::{ConnectionString, EndpointProtocol};
pub use self::connection_string_builder::ConnectionStringBuilder;
mod client_endpoint;
mod container_sas_builder;
mod hyper_client_endpoint;
pub mod shared_access_signature;
pub use client_endpoint::ClientEndpoint;
pub use hyper_client_endpoint::HyperClientEndpoint;

pub trait Client {
    fn blob_uri(&self) -> &str;
    fn table_uri(&self) -> &str;

    fn perform_request<F>(
        &self,
        uri: &str,
        method: &Method,
        headers_func: F,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder;

    fn perform_table_request<F>(
        &self,
        segment: &str,
        method: &Method,
        headers_func: F,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder;
}

pub trait ClientRequired<'a, C>
where
    C: Client,
{
    fn client(&self) -> &'a C;
}

pub trait KeyClientRequired<'a> {
    fn key_client(&self) -> &'a KeyClient;
}

pub trait SharedAccessSignatureSupport<'a> {
    type O;
    fn with_shared_access_signature(
        self,
        signature: &'a shared_access_signature::SharedAccessSignature,
    ) -> Self::O;
}

pub trait SharedAccessSignatureRequired<'a> {
    fn shared_access_signature(&self) -> &'a shared_access_signature::SharedAccessSignature;
}

#[derive(Debug, Clone, PartialEq)]
pub struct IPRange {
    pub start: std::net::IpAddr,
    pub end: std::net::IpAddr,
}

pub type CopyId = uuid::Uuid;

pub fn copy_id_from_headers(headers: &HeaderMap) -> Result<CopyId, AzureError> {
    let copy_id = headers
        .get_as_str(COPY_ID)
        .ok_or_else(|| AzureError::HeaderNotFound(COPY_ID.to_owned()))?;
    Ok(uuid::Uuid::parse_str(copy_id)?)
}
