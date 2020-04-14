use crate::from_headers::*;
use crate::ResourceQuota;
use crate::{Document, DocumentAttributes};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct ListAttachmentsResponse {}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListAttachmentsResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:#?}", headers);

        Ok(ListAttachmentsResponse {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
