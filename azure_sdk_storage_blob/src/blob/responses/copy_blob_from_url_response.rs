use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    content_md5_from_headers, date_from_headers, request_id_from_headers,
    request_server_encrypted_from_headers, RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBlobFromUrlResponse {
    //pub content_md5: [u8; 16],
//pub request_id: RequestId,
//pub date: DateTime<Utc>,
//pub request_server_encrypted: bool,
}

impl TryFrom<&HeaderMap> for CopyBlobFromUrlResponse {
    type Error = AzureError;
    fn try_from(value: &HeaderMap) -> Result<Self, Self::Error> {
        println!("value == {:#?}", value);
        Ok(Self {})
    }
}
