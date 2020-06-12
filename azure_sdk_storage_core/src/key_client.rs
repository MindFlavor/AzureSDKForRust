use crate::rest_client::{perform_request, ServiceType};
use crate::{Client, ClientEndpoint, ConnectionString, HyperClientEndpoint};
use azure_sdk_core::errors::AzureError;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use url::Url;

#[derive(Debug, Clone)]
pub struct KeyClient {
    account: String,
    key: String,
    sas_token: Option<Vec<(String, String)>>,
    hc: hyper::Client<HttpsConnector<hyper::client::HttpConnector>>,
    blob_uri: String,
    table_uri: String,
}

impl KeyClient {
    pub fn new(account: &str, key: &str) -> Result<KeyClient, AzureError> {
        KeyClient::azure(account, key)
    }

    pub fn azure_sas(account: &str, sas_token: &str) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());
        let params = KeyClient::get_sas_token_parms(sas_token);

        Ok(KeyClient {
            account: account.to_owned(),
            key: String::new(),
            sas_token: Some(params),
            hc: client,
            blob_uri: format!("https://{}.blob.core.windows.net", account),
            table_uri: format!("https://{}.table.core.windows.net", account),
        })
    }

    pub fn azure(account: &str, key: &str) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        Ok(KeyClient {
            account: account.to_owned(),
            key: key.to_owned(),
            sas_token: None,
            hc: client,
            blob_uri: format!("https://{}.blob.core.windows.net", account),
            table_uri: format!("https://{}.table.core.windows.net", account),
        })
    }

    pub fn from_connection_string(connection_string: &str) -> Result<Self, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        match ConnectionString::new(connection_string)? {
            ConnectionString {
                account_name: Some(account),
                account_key: Some(_),
                sas: Some(sas_token),
                ..
            } => {
                log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");
                Ok(KeyClient {
                    account: account.to_owned(),
                    key: String::new(),
                    sas_token: Some(KeyClient::get_sas_token_parms(sas_token)),
                    hc: client,
                    blob_uri: format!("https://{}.blob.core.windows.net", account),
                    table_uri: format!("https://{}.table.core.windows.net", account), 
                })
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                ..
            } => Ok(KeyClient {
                account: account.to_owned(),
                key: String::new(),
                sas_token: Some(KeyClient::get_sas_token_parms(sas_token)),
                hc: client,
                blob_uri: format!("https://{}.blob.core.windows.net", account),
                table_uri: format!("https://{}.table.core.windows.net", account), 
            }),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                ..
            } => Ok(KeyClient {
                account: account.to_owned(),
                key: key.to_owned(),
                sas_token: None,
                hc: client,
                blob_uri: format!("https://{}.blob.core.windows.net", account),
                table_uri: format!("https://{}.table.core.windows.net", account), 
            }),
            _ => {
                return Err(AzureError::GenericErrorWithText(
                    "Could not create a storage client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                        .to_owned(),
                ))
            }
        }
    }

    pub fn emulator(
        blob_storage_url: &Url,
        table_storage_url: &Url,
    ) -> Result<KeyClient, AzureError> {
        let client = hyper::Client::builder().build(HttpsConnector::new());

        let blob_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
        debug!("blob_uri == {}", blob_uri);
        let table_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
        debug!("table_uri == {}", table_uri);

        Ok(KeyClient {
            account: "devstoreaccount1".to_owned(),
            key: "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==".to_owned(),
            sas_token: None,
            hc: client,
            blob_uri,
            table_uri,
        })
    }

    fn add_sas_token_to_uri(&self, uri: &str) -> String {
        match &self.sas_token {
            Some(token) => Url::parse_with_params(uri, token).unwrap().to_string(),
            None => String::from(uri),
        }
    }

    fn get_sas_token_parms(sas_token: &str) -> Vec<(String, String)> {
        Url::options()
            // Any base url will do: we just need to parse the SAS token
            // to get its query pairs.
            .base_url(Some(&Url::parse("https://blob.core.windows.net").unwrap()))
            .parse(sas_token)
            .unwrap()
            .query_pairs()
            .map(|p| (String::from(p.0), String::from(p.1)))
            .collect()
    }

    /// Uri scheme + authority e.g. http://myaccount.table.core.windows.net/
    pub fn get_uri_prefix(&self, service_type: ServiceType) -> String {
        match service_type {
            ServiceType::Blob => format!("{}/", self.blob_uri()),
            ServiceType::Table => format!("{}/", self.table_uri()),
        }
    }
}

impl Client for KeyClient {
    #[inline]
    fn blob_uri(&self) -> &str {
        &self.blob_uri
    }

    #[inline]
    fn table_uri(&self) -> &str {
        &self.table_uri
    }

    fn perform_request<F>(
        &self,
        uri: &str,
        method: &Method,
        headers_func: F,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder,
    {
        let uri = self.add_sas_token_to_uri(uri);

        perform_request(
            self,
            &uri,
            method,
            headers_func,
            request_body,
            ServiceType::Blob,
        )
    }

    fn perform_table_request<F>(
        &self,
        segment: &str,
        method: &Method,
        headers_func: F,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(::http::request::Builder) -> ::http::request::Builder,
    {
        debug!("segment: {}, method: {:?}", segment, method,);

        let uri =
            self.add_sas_token_to_uri((self.get_uri_prefix(ServiceType::Table) + segment).as_str());

        perform_request(
            self,
            &uri,
            method,
            headers_func,
            request_str,
            ServiceType::Table,
        )
    }
}

impl ClientEndpoint for KeyClient {
    fn account(&self) -> &str {
        &self.account
    }

    fn key(&self) -> &str {
        &self.key
    }
}

impl HyperClientEndpoint for KeyClient {
    fn hyper_client(&self) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>> {
        &self.hc
    }
}
