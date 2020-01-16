use crate::{
    activity_id_from_headers, last_state_change_from_headers, request_charge_from_headers,
};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStoredProcedureResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub last_change: DateTime<Utc>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DeleteStoredProcedureResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            last_change: last_state_change_from_headers(headers)?,
        })
    }
}
