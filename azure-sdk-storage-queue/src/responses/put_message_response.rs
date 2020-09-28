//use crate::from_headers::*;
use azure_sdk_core::errors::AzureError;
//use azure_sdk_core::{
//    continuation_token_from_headers_optional, session_token_from_headers, SessionToken,
//};
//use chrono::{DateTime, Utc};
use hyper::header::HeaderMap;
//use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutMessageResponse {
    #[serde(rename = "QueueMessage")]
    pub queue_message: QueueMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMessage {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "InsertionTime")]
    pub insertion_time: String,
    #[serde(rename = "ExpirationTime")]
    pub expiration_time: String,
    #[serde(rename = "PopReceipt")]
    pub pop_receipt: String,
    #[serde(rename = "TimeNextVisible")]
    pub time_next_visible: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for PutMessageResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        println!("headers == {:?}", headers);

        let received = &std::str::from_utf8(body)?[3..];
        println!("receieved == {:#?}", received);
        let mut response: Self = serde_xml_rs::from_reader(&body[3..])?;

        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?><EnumerationResults ServiceEndpoint=\"https://azureskdforrust.queue.core.windows.net/\"><Prefix>a</Prefix><MaxResults>2</MaxResults><Queues><Queue><Name>azureiscool</Name></Queue><Queue><Name>azurerocks</Name></Queue></Queues><NextMarker /></EnumerationResults>";

        let response: PutMessageResponse = serde_xml_rs::from_str(range).unwrap();
    }
}
