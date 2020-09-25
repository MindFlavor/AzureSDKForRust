//use crate::from_headers::*;
use azure_sdk_core::errors::AzureError;
//use azure_sdk_core::{
//    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
//};
//use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;
//use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<u32>,

    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ListQueuesResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        println!("headers == {:?}", headers);

        let received = &std::str::from_utf8(body)?[3..];
        println!("receieved == {:#?}", received);
        let mut response: Self = serde_xml_rs::from_reader(&body[3..])?;

        // get rid of the ugly Some("") empty string
        // we use None as Rust dictates to identify
        // lack of value.
        if let Some(next_marker) = &response.next_marker {
            if next_marker == "" {
                response.next_marker = None;
            }
        }

        Ok(response)
    }
}
