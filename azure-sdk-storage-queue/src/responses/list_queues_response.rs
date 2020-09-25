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

//
//impl<T> std::convert::TryFrom<&[u8]> for ListDocumentsResponseEntities<T>
//where
//    T: DeserializeOwned,
//{
//    type Error = AzureError;
//    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
//        Ok(serde_json::from_slice(body)?)
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &'static str = "
{
    \"_rid\": \"3iNTAJKxVCk=\",
    \"Documents\": [
        {
            \"color\": \"red\",
            \"myvalue\": \"#f00\",
            \"id\": \"c5d11a65-2e5a-3d9f-4de8-2447259dff38\",
            \"_rid\": \"3iNTAJKxVCkBAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkBAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100eb0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        },
        {
            \"color\": \"yellow\",
            \"myvalue\": \"#ff0\",
            \"id\": \"894dd5ff-573e-f38a-b8c4-5eae5071c900\",
            \"_rid\": \"3iNTAJKxVCkCAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkCAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100ec0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        }
    ],
    \"_count\": 7
}";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct MyStruct {
        id: String,
        color: String,
        myvalue: String,
    }

    #[test]
    fn test_list_document() {
        let _document_attributes =
            serde_json::from_slice::<ListDocumentsResponseAttributes>(BODY.as_bytes()).unwrap();
        let _entries =
            serde_json::from_slice::<ListDocumentsResponseEntities<MyStruct>>(BODY.as_bytes())
                .unwrap();
    }
}
