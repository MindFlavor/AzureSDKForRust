use crate::from_headers::*;
use crate::ResourceQuota;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{etag_from_headers, session_token_from_headers};
use chrono::{DateTime, Utc};
use http::{HeaderMap, StatusCode};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateSlugAttachmentResponse {}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateSlugAttachmentResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", body);

        Ok(Self {})
    }
}
