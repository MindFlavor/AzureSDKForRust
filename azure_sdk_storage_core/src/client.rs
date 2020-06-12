use crate::key_client::get_sas_token_parms;
use crate::{ConnectionString, KeyClient};
use azure_sdk_core::errors::AzureError;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use url::Url;

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

    //
    // def impl
    //
    fn new(account: &str, key: &str) -> Result<KeyClient, AzureError> {
        KeyClient::azure(account, key)
    }

    fn azure_sas(account: &str, sas_token: &str) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());
        let params = get_sas_token_parms(sas_token);

        Ok(KeyClient::new(
            account.to_owned(),
            String::new(),
            Some(params),
            client,
            format!("https://{}.blob.core.windows.net", account),
            format!("https://{}.table.core.windows.net", account),
        ))
    }

    fn azure(account: &str, key: &str) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Ok(KeyClient::new(
            account.to_owned(),
            key.to_owned(),
            None,
            client,
            format!("https://{}.blob.core.windows.net", account),
            format!("https://{}.table.core.windows.net", account),
        ))
    }

    fn from_connection_string(connection_string: &str) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        match ConnectionString::new(connection_string)? {
            ConnectionString {
                account_name: Some(account),
                account_key: Some(_),
                sas: Some(sas_token),
                ..
            } => {
                log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");
                Ok(KeyClient::new(
                    account.to_owned(),
                    String::new(),
                    Some(get_sas_token_parms(sas_token)),
                    client,
                    format!("https://{}.blob.core.windows.net", account),
                    format!("https://{}.table.core.windows.net", account), 
                ))
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                ..
            } => Ok(KeyClient ::new(
                account.to_owned(),
                String::new(),
                Some(get_sas_token_parms(sas_token)),
                client,
                format!("https://{}.blob.core.windows.net", account),
                format!("https://{}.table.core.windows.net", account), 
            )),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                ..
            } => Ok(KeyClient::new(
                account.to_owned(),
                key.to_owned(),
                None,
                client,
                format!("https://{}.blob.core.windows.net", account),
                format!("https://{}.table.core.windows.net", account), 
            )),
            _ => {
                return Err(AzureError::GenericErrorWithText(
                    "Could not create a storage client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                        .to_owned(),
                ))
            }
        }
    }

    fn emulator(blob_storage_url: &Url, table_storage_url: &Url) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        let blob_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
        debug!("blob_uri == {}", blob_uri);
        let table_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
        debug!("table_uri == {}", table_uri);

        Ok(KeyClient::new(
            "devstoreaccount1".to_owned(),
            "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==".to_owned(),
            None,
            client,
            blob_uri,
            table_uri,
        ))
    }
}
