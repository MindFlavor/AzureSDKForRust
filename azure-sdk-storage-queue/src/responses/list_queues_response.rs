//use crate::from_headers::*;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{
    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
};
use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQueuesResponse {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub documents: Vec<String>,
}

impl std::convert::TryFrom<&[u8]> for ListQueuesResponse {
    type Error = AzureError;
    fn try_from(body: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(body)?)
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
